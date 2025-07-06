use crate::models::todo::TodoList;
use rusqlite::{params, Connection, Result, ToSql};

pub fn count(conn: &Connection, table_name: &str) -> Result<i64> {
    let mut stmt =
        conn.prepare("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1")?;
    let exists: i64 = stmt.query_row([table_name], |row| row.get(0))?;
    if exists == 0 {
        return Err(rusqlite::Error::InvalidQuery);
    }

    let sql = format!("SELECT COUNT(*) FROM {} WHERE done = 0", table_name);
    conn.query_row(&sql, [], |row| row.get(0))
}

pub fn add_todo(conn: &Connection, todo: &TodoList) -> Result<()> {
    conn.execute(
        "INSERT INTO todo (name, description, priority, done) VALUES (?1, ?2, ?3, ?4)",
        params![todo.name, todo.description, todo.priority, todo.done],
    )?;

    Ok(())
}

pub fn update_todo(
    conn: &Connection,
    id: i32,
    name: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    let mut updates = vec![];
    let mut values: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(name) = name {
        updates.push("name = ?");
        values.push(Box::new(name));
    }

    if let Some(description) = description {
        updates.push("description = ?");
        values.push(Box::new(description));
    }
    let param_refs: Vec<&dyn ToSql> = values.iter().map(|b| b.as_ref()).collect();

    let sql = format!("UPDATE todo SET {} where id = {id}", updates.join(", "));
    let mut stmt = conn.prepare(&sql)?;

    let row_affected = stmt.execute(param_refs.as_slice())?;

    if row_affected == 0 {
        return Err(rusqlite::Error::StatementChangedRows(0));
    }
    Ok(())
}

pub fn get_all_todo(conn: &Connection) -> Result<Vec<TodoList>> {
    let mut stmt = conn.prepare("SELECT * FROM todo")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(TodoList {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            priority: row.get(3)?,
            done: row.get(4)?,
        })
    })?;

    let mut result = Vec::new();
    for todo in todo_iter {
        result.push(todo?);
    }

    Ok(result)
}

pub fn get_todo_by_id(conn: &Connection, id: i32) -> Result<TodoList> {
    let sql = format!("SELECT * FROM todo WHERE id = {id}");
    conn.query_row(&sql, [], |row| {
        Ok(TodoList {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            priority: row.get(3)?,
            done: row.get(4)?,
        })
    })
}

pub fn get_undone_todo(conn: &Connection) -> Result<Vec<TodoList>> {
    let mut stmt = conn.prepare("SELECT * FROM todo WHERE done = 0")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(TodoList {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            priority: row.get(3)?,
            done: row.get(4)?,
        })
    })?;

    let todos: Result<Vec<TodoList>> = todo_iter.collect();
    todos
}

pub fn get_done_todo(conn: &Connection) -> Result<Vec<TodoList>> {
    let mut stmt = conn.prepare("SELECT * FROM todo WHERE done = 1")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(TodoList {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            priority: row.get(3)?,
            done: row.get(4)?,
        })
    })?;

    let todos: Result<Vec<TodoList>> = todo_iter.collect();
    todos
}

pub fn done(conn: &Connection, id: i32) -> Result<()> {
    let todo = get_todo_by_id(conn, id);
    if todo.unwrap().done == 1 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let sql = format!("UPDATE todo SET done = 1 WHERE id = {}", id);
    let mut stmt = conn.prepare(&sql)?;
    let row_affected = stmt.execute([])?;

    if row_affected == 0 {
        return Err(rusqlite::Error::StatementChangedRows(0));
    }

    Ok(())
}
