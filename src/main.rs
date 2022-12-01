use clap::Parser;
use cli::{Cli, Commands};
use std::error::Error;

mod cli;
mod cmd;
mod data;
mod output;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let output = args.output.to_output();

    match &args.command {
        Commands::Info(arguments) => cmd::info::run(arguments, output.as_ref()),
        Commands::Diff(arguments) => cmd::diff::run(arguments, output.as_ref()),
        Commands::List(command) => cmd::list::run(command, output.as_ref()),
    }

    Ok(())
}
