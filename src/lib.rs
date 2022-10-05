pub use crate::cli::*;
pub use crate::error::*;
pub use crate::project::*;
pub use crate::commands::*;

pub mod cli;
pub mod error;
pub mod project;
pub mod commands;

pub async fn run() -> Result<(), Error> {
    let args = Arguments::new()?;
    args.print();

    match args.get_command() {
        "ahoy" => commands::init::init().await?,
        "employ" => commands::pkg::add_package(&args.get_flag("source").unwrap().value)?,
        "destroy" => commands::pkg::remove_package(&args.get_flag("name").unwrap().value)?,
        _ => println!("Unknown command"),
    }

    Ok(())
}