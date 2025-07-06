use database::{
    job::{get_all_todo, get_done_todo, get_todo_by_id, get_undone_todo},
    models::todo::TodoList,
};
use rusqlite::{Connection, Result};

use console::style;

pub fn show_all(conn: &Connection) -> Result<()> {
    match get_all_todo(conn) {
        Ok(values) => {
            for (idx, val) in values.iter().enumerate() {
                print_priority_color(idx, val)?;
            }
        }
        Err(err) => eprintln!("Error querying: {}", err),
    }

    Ok(())
}

pub fn show_undone(conn: &Connection) -> Result<()> {
    match get_undone_todo(conn) {
        Ok(values) => {
            for (idx, val) in values.iter().enumerate() {
                print_priority_color(idx, val)?;
            }
        }
        Err(err) => eprintln!("Error querying: {}", err),
    }

    Ok(())
}

pub fn show_done(conn: &Connection) -> Result<()> {
    match get_done_todo(conn) {
        Ok(values) => {
            for (idx, val) in values.iter().enumerate() {
                println!("{}. {}", idx + 1, style(val).green());
            }
        }
        Err(err) => eprintln!("Error querying: {}", err),
    }

    Ok(())
}

pub fn show_todo_by_id(conn: &Connection, id: i32) {
    let todo = get_todo_by_id(conn, id);
    println!("{}", todo.unwrap());
}

fn print_priority_color(idx: usize, todo: &TodoList) -> Result<()> {
    let todo_done: i32 = todo.done;
    if todo_done == 1 {
        println!("{}", style(&todo).green())
    } else {
        match &todo.priority {
            1 => println!("{}. {}", idx + 1, style(todo).cyan()),
            2 => println!("{}. {}", idx + 1, style(todo).yellow()),
            3 => println!("{}. {}", idx + 1, style(todo).color256(208)),
            4 => println!("{}. {}", idx + 1, style(todo).red()),
            0_u8 | 5_u8..=u8::MAX => todo!(),
        }
    }

    Ok(())
}
