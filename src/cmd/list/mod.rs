use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    str,
};

use clap::Subcommand;
use object::{
    pe::{ImageNtHeaders32, ImageNtHeaders64},
    read::pe::{self, ImageNtHeaders},
    Object,
};
use symbolic::{
    common::Name,
    demangle::{Demangle, DemangleOptions},
};

use crate::output::PrintOutput;

use self::data::*;

pub mod data;

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

// TODO(MSt): Move get_* methods into a separate "data" module. Naming needs to be determined
impl Commands {
    fn get_dependencies(&self, file: &Path) -> Result<Vec<Dependency>, Box<dyn Error>> {
        let mut dependencies = Vec::<Dependency>::new();
        let binary_data = fs::read(file)?;
        let object_file = object::File::parse(&*binary_data)?;

        let imports = object_file.imports()?;
        let mut last_library = "";
        for import in imports {
            let library = str::from_utf8(import.library()).unwrap();

            if library != last_library {
                dependencies.push(Dependency {
                    library: library.to_owned(),
                });
            }

            last_library = library;
        }

        Ok(dependencies)
    }

    fn get_imports(&self, file: &Path) -> Result<Vec<Import>, Box<dyn Error>> {
        let mut import_results = Vec::<Import>::new();
        let binary_data = fs::read(file)?;
        let object_file = object::File::parse(&*binary_data)?;

        let imports = object_file.imports()?;
        for import in imports {
            let library = str::from_utf8(import.library()).unwrap();
            let function_name = str::from_utf8(import.name()).unwrap();

            let demangled_name = Name::from(function_name);
            let demangled_name = demangled_name.try_demangle(DemangleOptions::complete());

            import_results.push(Import {
                library: library.to_owned(),
                function: function_name.to_owned(),
                function_demangled: demangled_name.to_string(),
            });
        }

        Ok(import_results)
    }

    fn get_exports(&self, file: &Path) -> Result<Vec<Export>, Box<dyn Error>> {
        let binary_data = fs::read(file)?;

        Ok(match object::FileKind::parse(&*binary_data)? {
            object::FileKind::Pe32 => self.get_exports_pe::<ImageNtHeaders32>(&binary_data)?,
            object::FileKind::Pe64 => self.get_exports_pe::<ImageNtHeaders64>(&binary_data)?,
            _ => self.get_exports_default(&binary_data)?,
        })
    }

    fn get_exports_default(&self, binary_data: &Vec<u8>) -> Result<Vec<Export>, Box<dyn Error>> {
        let mut exports_result = Vec::<Export>::new();
        let object_file = object::File::parse(&**binary_data)?;

        let exports = object_file.exports()?;
        for export in exports {
            let function_name = str::from_utf8(export.name()).unwrap();

            let demangled_name = Name::from(function_name);
            let demangled_name = demangled_name.try_demangle(DemangleOptions::complete());

            exports_result.push(Export {
                address: Some(export.address()),
                function: function_name.to_owned(),
                function_demangled: demangled_name.to_string(),
                target: None,
            });
        }

        Ok(exports_result)
    }

    fn get_exports_pe<T: ImageNtHeaders>(
        &self,
        binary_data: &Vec<u8>,
    ) -> Result<Vec<Export>, Box<dyn Error>> {
        let mut exports_result = Vec::<Export>::new();
        let object_file = object::read::pe::PeFile::<T>::parse(&**binary_data)?;

        if let Some(export_table) = object_file.export_table()? {
            for export in export_table.exports()? {
                let function_name = str::from_utf8(export.name.unwrap_or_default()).unwrap();

                let demangled_name = Name::from(function_name);
                let demangled_name = demangled_name.try_demangle(DemangleOptions::complete());

                match export.target {
                    pe::ExportTarget::Address(address) => {
                        exports_result.push(Export {
                            address: Some(address.into()),
                            function: function_name.to_owned(),
                            function_demangled: demangled_name.to_string(),
                            target: None,
                        });
                    }
                    pe::ExportTarget::ForwardByName(dll, name) => {
                        exports_result.push(Export {
                            address: None,
                            function: function_name.to_owned(),
                            function_demangled: demangled_name.to_string(),
                            target: Some(ExportTarget {
                                library: str::from_utf8(dll).unwrap_or_default().to_owned(),
                                forward: ForwardType::Name(
                                    str::from_utf8(name).unwrap_or_default().to_owned(),
                                ),
                            }),
                        });
                    }
                    pe::ExportTarget::ForwardByOrdinal(dll, ordinal) => {
                        exports_result.push(Export {
                            address: None,
                            function: function_name.to_owned(),
                            function_demangled: demangled_name.to_string(),
                            target: Some(ExportTarget {
                                library: str::from_utf8(dll).unwrap_or_default().to_owned(),
                                forward: ForwardType::Ordinal(ordinal.into()),
                            }),
                        })
                    }
                }
            }
        }

        Ok(exports_result)
    }

    fn list_dependencies(
        &self,
        file: &Path,
        output: &dyn PrintOutput,
    ) -> Result<(), Box<dyn Error>> {
        self.get_dependencies(file)?
            .iter()
            .for_each(|item| output.print_dependency(item));

        Ok(())
    }

    fn list_exports(&self, file: &Path, output: &dyn PrintOutput) -> Result<(), Box<dyn Error>> {
        self.get_exports(file)?
            .iter()
            .for_each(|item| output.print_export(item));

        Ok(())
    }

    fn list_imports(&self, file: &Path, output: &dyn PrintOutput) -> Result<(), Box<dyn Error>> {
        self.get_imports(file)?
            .iter()
            .for_each(|item| output.print_import(item));

        Ok(())
    }
}
