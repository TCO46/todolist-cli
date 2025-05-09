use rusqlite::{Connection, Result};

pub fn count(conn: &Connection, table_name: &str) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1")?;
    let exists: i64 = stmt.query_row([table_name], |row| row.get(0))?;
    if exists == 0 {
        return Err(rusqlite::Error::InvalidQuery)
    }

    let sql = format!("SELECT COUNT(*) FROM {}", table_name);
    conn.query_row(&sql, [], |row| row.get(0))
}
