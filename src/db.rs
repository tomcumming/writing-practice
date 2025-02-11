use std::marker::PhantomData;

use rusqlite::{Connection, OpenFlags, ToSql};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Id<T> {
    _type: PhantomData<T>,
    id: u64,
}

impl<T> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.id, f)
    }
}

impl<T> Id<T> {
    pub fn new(id: u64) -> Self {
        Id {
            _type: PhantomData,
            id,
        }
    }

    pub fn get(self) -> u64 {
        self.id
    }
}

impl<T> ToSql for Id<T> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.id.to_sql()
    }
}

pub struct Document;
pub struct Dict;

#[derive(Debug)]
pub struct WordDef {
    pub simplified: String,
    pub traditional: String,
    pub pinyin: Vec<String>,
    pub defs: Vec<String>,
}

pub struct Db {
    conn: Connection,
}

fn string_error<E: ToString>(err: E) -> String {
    err.to_string()
}

impl Db {
    pub fn open() -> Result<Db, String> {
        let conn = Connection::open_with_flags(
            "writer.sqlite",
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        )
        .map_err(|_| "Could not open writer.sqlite")?;

        for stmt in SQL_SCHEMA {
            conn.prepare(stmt)
                .map_err(string_error)?
                .execute([])
                .map_err(string_error)?;
        }

        Ok(Db { conn })
    }

    pub fn get_document(&self, name: &str) -> Result<Id<Document>, String> {
        let sql_insert = r#"
insert into document (name) values(?1)
  on conflict(name) do nothing
        "#;

        self.conn
            .prepare(sql_insert)
            .map_err(string_error)?
            .execute([name])
            .map_err(string_error)?;
        self.conn
            .prepare(r#"select rowid from document where name = ?1"#)
            .map_err(string_error)?
            .query_row([name], |row| row.get(0).map(Id::new))
            .map_err(string_error)
    }

    pub fn get_dict(&self, name: &str) -> Result<Id<Dict>, String> {
        let sql_insert = r#"
insert into dict (name) values(?1)
  on conflict(name) do nothing
        "#;

        self.conn
            .prepare(sql_insert)
            .map_err(string_error)?
            .execute([name])
            .map_err(string_error)?;
        self.conn
            .prepare(r#"select rowid from dict where name = ?1"#)
            .map_err(string_error)?
            .query_row([name], |row| row.get(0).map(Id::new))
            .map_err(string_error)
    }

    pub fn replace_dictionary(
        &mut self,
        name: &str,
        defs: impl Iterator<Item = WordDef>,
    ) -> Result<(), String> {
        let id = self.get_dict(name)?;
        let sql_delete = r#"delete from word_def where dict = ?1"#;
        self.conn
            .prepare(sql_delete)
            .map_err(string_error)?
            .execute([&id])
            .map_err(string_error)?;

        let tx = self.conn.transaction().map_err(string_error)?;

        for def in defs {
            let json = serde_json::to_value([def.pinyin, def.defs]).map_err(string_error)?;
            let json_str = serde_json::to_string(&json).map_err(string_error)?;

            let sql_insert = r#"
insert into word_def (dict, simplified, traditional, data)
    values (?1, ?2, ?3, ?4)"#;

            tx.prepare(sql_insert)
                .map_err(string_error)?
                .execute((&id, def.simplified, def.traditional, json_str))
                .map_err(string_error)?;
        }

        tx.commit().map_err(string_error)?;
        Ok(())
    }
}

const SQL_SCHEMA: [&str; 3] = [
    r#"
create table if not exists document (
  name text not null unique
) strict;"#,
    r#"
create table if not exists dict (
  name text not null unique
) strict;"#,
    r#"
create table if not exists word_def (
    dict integer not null,
    simplified text not null,
    traditional text not null,
    data text not null
) strict;"#,
];
