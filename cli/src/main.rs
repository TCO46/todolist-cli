use database;
use crate::database::models::todo::TodoList;
use rusqlite::Result;
use clap::{Parser, Subcommand};
const DATABASE_PATH: &str = "test.db";

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Count the numbers of to do
    Count,

    // Add new to do
    Add { 
        name: String,

        #[arg(short, long, default_value = "none")]
        description: String
    },

    // Show all to do list
    Show {
        // Show all the todo list (include undone and done)
        #[arg(short, long, num_args(0))]
        all: bool,

        // Show all the done todo list
        #[arg(short, long, num_args(0))]
        done: bool
    },

    // Update status of todo to done
    Done { id: i32 }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = database::connect_db(DATABASE_PATH)?;
    database::init_db(&conn)?;

    match &cli.command {
        Some(Commands::Count) => {
            println!("{:?}", database::job::count(&conn, "todo").unwrap());
        }
        Some(Commands::Add { name, description }) => {
            let new_todo = TodoList::new(name, description);
            database::job::add_todo(&conn, &new_todo)?;

            println!("Added '{}'", name);
        }
        Some(Commands::Show { all, done }) => {
            match (all, done) {
                (true, _) => {
                    utils::show_all(&conn)?;
                }
                (false, true) => {
                    utils::show_done(&conn)?;
                }
                (false, false) => {
                    utils::show_undone(&conn)?;
                }
            }
        }
        Some(Commands::Done { id }) => {
            database::job::done(&conn, *id)?;
            println!("Done {}", id);
        }
        None => {
            let todos = database::job::show_undone(&conn)?;

            for todo in todos {
                println!("{:?}", todo);
            }
        }
    }

    Ok(())
}
