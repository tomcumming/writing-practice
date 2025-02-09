use rusqlite::{Connection, OpenFlags};

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn open() -> Result<Db, String> {
        let conn = Connection::open_with_flags(
            "writer.sqlite",
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        )
        .map_err(|_| "Could not open writer.sqlite")?;

        conn.prepare(sql_schema)
            .map_err(|e| e.to_string())?
            .execute([])
            .map_err(|e| e.to_string())?;

        Ok(Db { conn })
    }

    pub fn get_document(&self, name: &str) -> Result<u64, String> {
        let sql_insert = r#"
insert into document (name) values(?1)
  on conflict(name) do nothing
        "#;
        let sql_select = r#"select rowid from document where name = ?1"#;

        self.conn
            .prepare(sql_insert)
            .map_err(|e| e.to_string())?
            .execute([name])
            .map_err(|e| e.to_string())?;
        self.conn
            .prepare(sql_select)
            .map_err(|e| e.to_string())?
            .query_row([name], |row| row.get(0))
            .map_err(|e| e.to_string())
    }
}

// pub fn get_or_create_document(name: &str) -> Result<

const sql_schema: &str = r##"
create table if not exists document (
  name text not null unique
) strict;
"##;
