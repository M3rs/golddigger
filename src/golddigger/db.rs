use rusqlite::{params, Connection, Result};
use std::collections::HashSet;

/// Database
pub struct Database {
    con: Connection,
}

impl Database {
    //! Creates a new Database and sets up the required table
    pub fn new(con_str: &str) -> Result<Database> {
        let con = Connection::open(con_str)?;
        con.execute(
            r#"CREATE TABLE IF NOT EXISTS codes (
                        id INTEGER PRIMARY KEY,
                        code TEXT NOT NULL
                    )"#,
            params![],
        )?;

        Ok(Database { con: con })
    }

    pub fn get_used(&self) -> Result<HashSet<String>> {
        let mut stmt = self.con.prepare("SELECT code FROM codes")?;
        let mut rows = stmt.query(params![])?;

        let mut used = HashSet::new();
        while let Some(row) = rows.next()? {
            used.insert(row.get(0)?);
        }

        Ok(used)
    }

    pub fn add_codes(&self, codes: &[String]) -> Result<()> {
        let mut insert = self.con.prepare("INSERT INTO codes (code) VALUES (?1);")?;
        for code in codes {
            println!("{}", code);
            insert.execute(params![code])?;
        }

        Ok(())
    }
}
