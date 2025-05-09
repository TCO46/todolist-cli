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

}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = database::connect_db(DATABASE_PATH)?;
    database::init_db(&conn)?;

    match &cli.command {
        Some(Commands::Count) => {
            println!("{:?}", database::job::count(&conn, "todo").unwrap());
        }
        None => {
            println!("Do nothing yet");
        }
    }

    Ok(())
}
