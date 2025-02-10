use std::marker::PhantomData;

use rusqlite::{Connection, OpenFlags};

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

        conn.prepare(SQL_SCHEMA)
            .map_err(string_error)?
            .execute([])
            .map_err(string_error)?;

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
            .prepare(r#"select rowid from document where name = ?1"#)
            .map_err(string_error)?
            .query_row([name], |row| row.get(0).map(Id::new))
            .map_err(string_error)
    }
}

const SQL_SCHEMA: &str = r##"
create table if not exists document (
  name text not null unique
) strict;

create table if not exists dict (
  name text not null unique
) strict;

create table if not exists word_def {
    simplified text not null,
    traditional text not null,
    data text not null
}
"##;
