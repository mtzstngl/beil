use clap::{Parser, Subcommand};

use crate::{cmd::*, output};

#[derive(Parser)]
#[clap(about, long_about = None, version)]
pub struct Cli {
    /// The Commands enumeration that specifies the subcommands.
    #[command(subcommand)]
    pub command: Commands,

    /// Selects the output format.
    #[arg(long, default_value_t = output::OutputType::Plain, value_enum)]
    pub output: output::OutputType,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Displays information, such as architecture of a binary.
    Info(info::Arguments),

    /// Compares two binaries and highlights their differences.
    Diff(diff::Arguments),

    /// List different parts of a given binary.
    #[clap(subcommand)]
    List(list::Commands),
}
