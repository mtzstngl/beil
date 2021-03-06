use crate::cmd::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, long_about = None, version)]
pub struct Cli {
    /// The Commands enumeration that specifies the subcommands.
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Displays information, such as architecture of a binary.
    Info(info::Arguments),

    /// List different parts of a given binary.
    #[clap(subcommand)]
    List(list::Commands),
}
