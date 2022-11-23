use crate::{cmd::*, output::OutputType};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, long_about = None, version)]
pub struct Cli {
    /// The Commands enumeration that specifies the subcommands.
    #[command(subcommand)]
    pub command: Commands,

    /// Selects the output format.
    #[arg(long, default_value_t = OutputType::Plain, value_enum)]
    pub output: OutputType,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Displays information, such as architecture of a binary.
    Info(info::Arguments),

    /// List different parts of a given binary.
    #[clap(subcommand)]
    List(list::Commands),
}
