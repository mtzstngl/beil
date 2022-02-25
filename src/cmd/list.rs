use clap::Subcommand;
use object::Object;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, str};
use symbolic::common::Name;
use symbolic::demangle::{Demangle, DemangleOptions};

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

pub fn run(command: &Commands) {
    match command {
        Commands::Dependencies { file } => command.list_dependencies(file),
        Commands::Exports { file } => command.list_exports(file),
        Commands::Imports { file } => command.list_imports(file),
    }
    .unwrap()
}

impl Commands {
    fn list_dependencies(&self, file: &Path) -> Result<(), Box<dyn Error>> {
        let binary_data = fs::read(file)?;
        let object_file = object::File::parse(&*binary_data)?;

        let imports = object_file.imports()?;
        let mut last_library = "";
        for import in imports {
            let library = str::from_utf8(import.library()).unwrap();

            if library != last_library {
                println!("{}", library);
            }

            last_library = library;
        }

        Ok(())
    }

    fn list_exports(&self, file: &Path) -> Result<(), Box<dyn Error>> {
        let binary_data = fs::read(file)?;
        let object_file = object::File::parse(&*binary_data)?;

        let exports = object_file.exports()?;
        for export in exports {
            let function_name = str::from_utf8(export.name()).unwrap();

            let demangled_name = Name::from(function_name);
            let demangled_name = demangled_name.try_demangle(DemangleOptions::complete());
            println!(
                "{:#x}: {} {}",
                export.address(),
                function_name,
                demangled_name
            );
        }

        Ok(())
    }

    fn list_imports(&self, file: &Path) -> Result<(), Box<dyn Error>> {
        let binary_data = fs::read(file)?;
        let object_file = object::File::parse(&*binary_data)?;

        let imports = object_file.imports()?;
        for import in imports {
            let library = str::from_utf8(import.library()).unwrap();
            let function_name = str::from_utf8(import.name()).unwrap();

            let demangled_name = Name::from(function_name);
            let demangled_name = demangled_name.try_demangle(DemangleOptions::complete());
            println!("{}: {} {}", library, function_name, demangled_name);
        }

        Ok(())
    }
}
