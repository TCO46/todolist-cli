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
    Show
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
            let test = database::job::add_todo(&conn, &new_todo)?;
            println!("{:?}", test);
        }
        Some(Commands::Show) => {
            let todos = database::job::show_all(&conn)?;

            for todo in todos {
                println!("{:?}", todo);
            }
        }
        None => {
            println!("Do nothing yet");
        }
    }

    Ok(())
}
