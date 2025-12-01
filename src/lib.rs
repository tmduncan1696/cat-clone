use std::error::Error;

pub mod commands;
use crate::commands::CatCommands;

pub mod cli;
use crate::cli::Cli;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let commands = CatCommands::from_cli(&cli).modify_lines();

    commands.print_lines();

    Ok(())
}
