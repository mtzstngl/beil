use clap::Parser;
use cli::{Cli, Commands};
use std::error::Error;

mod cli;
mod cmd;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match &args.command {
        Commands::Info(arguments) => cmd::info::run(arguments),
        Commands::List(command) => cmd::list::run(command),
    }

    Ok(())
}
