use std::{error::Error, path::PathBuf};

use clap::Args;

use crate::{data::info::Information, output::PrintOutput};

// Command line arguments for the info module.
#[derive(Args)]
pub struct Arguments {
    /// The file of which to display information.
    file: PathBuf,
}

pub fn run(arguments: &Arguments, output: &dyn PrintOutput) -> Result<(), Box<dyn Error>> {
    output.print_information(&Information::read(&arguments.file)?);
    Ok(())
}