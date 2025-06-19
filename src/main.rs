use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::*;

mod cli;
mod config;
mod templates;
mod project;
mod utils;

use cli::Commands;

#[derive(Parser)]
#[command(name = "murex")]
#[command(about = "A tool for creating and managing CLI utilities")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Init { name, template } => {
            cli::init_project(name, template)?;
        }
        Commands::List => {
            cli::list_projects()?;
        }
        Commands::Build { name } => {
            cli::build_project(name)?;
        }
        Commands::Remove { name } => {
            cli::remove_project(name)?;
        }
        Commands::Template { action } => {
            cli::handle_template_command(action)?;
        }
        Commands::Config { action } => {
            cli::handle_config_command(action)?;
        }
    }
    
    Ok(())
}
