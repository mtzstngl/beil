use std::{
    error::Error,
    path::{Path, PathBuf},
    str,
};

use clap::Subcommand;

use crate::data::{dependency::Dependency, export::Export, import::Import};
use crate::output::PrintOutput;

#[derive(Subcommand)]
pub enum Commands {
    /// List all the libraries that the binary depends on.
    Dependencies {
        /// The file from which to list the dependencies.
        file: PathBuf,
    },

    /// List all the exports of the given binary.
    Exports {
        /// The file from which to list the exports.
        file: PathBuf,
    },

    /// List all the imports of the given binary.
    Imports {
        /// The file from which to list the imports.
        file: PathBuf,
    },
}

pub fn run(command: &Commands, output: &dyn PrintOutput) {
    match command {
        Commands::Dependencies { file } => command.list_dependencies(file, output),
        Commands::Exports { file } => command.list_exports(file, output),
        Commands::Imports { file } => command.list_imports(file, output),
    }
    .unwrap()
}

impl Commands {
    fn list_dependencies(
        &self,
        file: &Path,
        output: &dyn PrintOutput,
    ) -> Result<(), Box<dyn Error>> {
        Dependency::read(file)?
            .iter()
            .for_each(|item| output.print_dependency(item));

        Ok(())
    }

    fn list_exports(&self, file: &Path, output: &dyn PrintOutput) -> Result<(), Box<dyn Error>> {
        Export::read(file)?
            .iter()
            .for_each(|item| output.print_export(item));

        Ok(())
    }

    fn list_imports(&self, file: &Path, output: &dyn PrintOutput) -> Result<(), Box<dyn Error>> {
        Import::read(file)?
            .iter()
            .for_each(|item| output.print_import(item));

        Ok(())
    }
}
