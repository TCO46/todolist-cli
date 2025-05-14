use database::models::todo::TodoList;

mod utils;
use rusqlite::Result;
use clap::{Parser, Subcommand};
const DATABASE_PATH: &str = "todo.db";

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Count the numbers of to do
    Count,

    /// Add new to do
    Add { 
        name: String,

        #[arg(short, long, default_value = "none")]
        description: String
    },

    /// Show all to do list
    Show {
        /// Show all the todo list (include undone and done)
        #[arg(short, long, num_args(0))]
        all: bool,

        /// Show all the done todo list
        #[arg(short, long, num_args(0))]
        done: bool,
    },
    
    /// Show a to do using id
    Id {
        id: i32
    },

    /// Update status of todo to done
    Done { id: i32 },

    /// Update Name or Description
    Update {
        id: i32,
        /// Specify name to update
        #[arg(short, long)]
        name: Option<String>,
        
        /// Specify description to update
        #[arg(short, long)]
        description: Option<String>,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = database::connect_db(DATABASE_PATH)?;
    database::init_db(&conn)?;

    match cli.command {
        Commands::Count => {
            println!("{:?}", database::job::count(&conn, "todo").unwrap());
        }
        Commands::Add { name, description } => {
            let new_todo = TodoList::new(&name, &description);
            database::job::add_todo(&conn, &new_todo)?;

            println!("Added '{}'", name);
        }
        Commands::Show { all, done } => {
            if let true = all {
                utils::show_all(&conn)?;
            } else if let true = done {
                utils::show_done(&conn)?;
            } else {
                utils::show_undone(&conn)?;
            }
        }
        Commands::Id { id } => {
        utils::show_todo_by_id(&conn, id);
        }
        Commands::Done { id } => {
            database::job::done(&conn, id)?;
            println!("Done {}", id);
        }
        Commands::Update { id, name, description } => {
            database::job::update_todo(&conn, id, name.as_deref(), description.as_deref())?;
        }
    }

    Ok(())
}
