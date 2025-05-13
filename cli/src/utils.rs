use database::job::{get_all_todo, get_done_todo, get_undone_todo};
use rusqlite::{Connection, Result};

pub fn show_all(conn: &Connection) -> Result<()> {
    let todos = get_all_todo(&conn);
    if let Ok(todo) = todos {
        println!("{:?}", todo);
    }

    Ok(())
}

pub fn show_undone(conn: &Connection) -> Result<()> {
    let todos = get_undone_todo(&conn);
    if let Ok(todo) = todos {
        println!("{:?}", todo);
    }

    Ok(())
}

pub fn show_done(conn: &Connection) -> Result<()> {
    let todos = get_done_todo(&conn);
    if let Ok(todo) = todos {
        println!("{:?}", todo);
    }

    Ok(())
}
