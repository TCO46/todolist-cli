use database::job::{get_all_todo, get_done_todo, get_todo_by_id, get_undone_todo};
use rusqlite::{Connection, Result};

pub fn show_all(conn: &Connection) -> Result<()> {
    match get_all_todo(&conn) {
        Ok(values) => {
            for val in values {
                println!("{}", val);
            }
        }
        Err(err) => eprintln!("Error querying: {}", err),
    }

    Ok(())
}

pub fn show_undone(conn: &Connection) -> Result<()> {
    match get_undone_todo(&conn) {
        Ok(values) => {
            for val in values {
                println!("{}", val);
            }
        }
        Err(err) => eprintln!("Error querying: {}", err),
    }

    Ok(())
}

pub fn show_done(conn: &Connection) -> Result<()> {
    match get_done_todo(&conn) {
        Ok(values) => {
            for val in values {
                println!("{}", val);
            }
        }
        Err(err) => eprintln!("Error querying: {}", err),
    }

    Ok(())
}

pub fn show_todo_by_id(conn: &Connection, id: i32) {
    let todo = get_todo_by_id(&conn, id);
    println!("{:?}", todo.unwrap());
}
