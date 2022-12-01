use std::path::PathBuf;

use clap::Args;

use crate::output::PrintOutput;

// Command line arguments for the info module.
#[derive(Args)]
pub struct Arguments {
    /// The old file which is compared to the new file.
    old_file: PathBuf,

    /// The new file which is compared to the old file.
    new_file: PathBuf,
}

pub fn run(arguments: &Arguments, output: &dyn PrintOutput) {
    // TODO(MSt): Use the data from the future data module and use that to compare
    unimplemented!("TODO")
}
