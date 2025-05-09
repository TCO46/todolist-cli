use rusqlite::{params, Connection, Result};
use crate::models::todo::TodoList;

pub fn count(conn: &Connection, table_name: &str) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1")?;
    let exists: i64 = stmt.query_row([table_name], |row| row.get(0))?;
    if exists == 0 {
        return Err(rusqlite::Error::InvalidQuery)
    }

    let sql = format!("SELECT COUNT(*) FROM {}", table_name);
    conn.query_row(&sql, [], |row| row.get(0))
}

pub fn add_todo(conn: &Connection, todo: &TodoList) -> Result<()> {
    conn.execute(
        "INSERT INTO todo (name, description, done) VALUES (?1, ?2, ?3)",
        params![todo.name, todo.description, todo.done],
    )?;

    Ok(())
}

pub fn show_all(conn: &Connection) -> Result<Vec<TodoList>> {
    let mut stmt = conn.prepare("SELECT * FROM todo")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(TodoList {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            done: row.get(3)?
        })
    })?;

    let todos: Result<Vec<TodoList>> = todo_iter.collect();
    todos
}
