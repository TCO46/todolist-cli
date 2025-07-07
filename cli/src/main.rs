use database::models::todo::TodoList;
use database::models::sort::Sort;

mod utils;
use clap::{Parser, Subcommand};
use rusqlite::Result;
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
        /// Add name to todo
        name: String,
        /// Add description to todo
        #[arg(short, long, default_value = "none")]
        description: String,

        /// Add priority to todo from 1-4
        #[arg(short, long, default_value_t = 1)]
        priority: u8,
    },

    /// Show all to do list
    Show {
        /// Show all the todo list (include undone and done)
        #[arg(short, long, num_args(0))]
        all: bool,

        /// Show all the done todo list
        #[arg(short, long, num_args(0))]
        done: bool,

        /// Show todo by id
        #[arg(long)]
        id: Option<i32>,
      
        /// Sort todo
        #[clap(value_enum)]
        sort: Option<Sort>
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
    },

    ///Delete a todo
    Delete {
        id: i32
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
        Commands::Add {
            name,
            description,
            priority,
        } => {
            let new_todo = TodoList::new(&name, &description, priority);
            database::job::add_todo(&conn, &new_todo)?;

            println!("Added '{}'", name);
        }
        Commands::Show { all, done, id, sort } => {
            if all {
                utils::show_all(&conn, sort)?;
            } else if done {
                utils::show_done(&conn)?;
            } else if let Some(i) = id {
                utils::show_todo_by_id(&conn, i);
            } else {
                utils::show_undone(&conn, sort)?;
            }
        }
        Commands::Done { id } => {
            database::job::done(&conn, id)?;
            println!("Done {}", id);
        }
        Commands::Update {
            id,
            name,
            description,
        } => {
            database::job::update_todo(&conn, id, name.as_deref(), description.as_deref())?;

            if let Some(n) = name {
                println!("TODO (ID {id:?}) name updated with: {n:?}");
            } else if let Some(d) = description {
                println!("TODO (ID {id:?}) description updated with: {d:?}");
            }
        }
        Commands::Delete { id } => {
            database::job::delete_todo(&conn, id)?;
            println!("Deleted ({id})")
        }
    }

    Ok(())
}
