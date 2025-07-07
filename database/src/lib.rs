use rusqlite::{Connection, Result};

pub mod job;
pub mod models;

pub fn connect_db(path: &str) -> Result<Connection> {
    Connection::open(path)
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  description     TEXT DEFAULT 'none',
                  priority        INTEGER,
                  done            INTEGER
                  )",
        [],
    )?;

    Ok(())
}
