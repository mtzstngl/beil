use std::{error::Error, fs, path::Path, str};

use object::{
    pe::{ImageNtHeaders32, ImageNtHeaders64},
    read::pe::{self, ImageNtHeaders},
    Object,
};
use symbolic::{
    common::Name,
    demangle::{Demangle, DemangleOptions},
};

pub struct Export {
    pub address: Option<u64>,
    pub function: String,
    pub function_demangled: String,
    pub target: Option<ExportTarget>,
}

pub struct ExportTarget {
    pub library: String,
    pub forward: ForwardType,
}

pub enum ForwardType {
    Name(String),
    Ordinal(u64),
}

impl Export {
    pub fn read(file: &Path) -> Result<Vec<Export>, Box<dyn Error>> {
        let binary_data = fs::read(file)?;

        Ok(match object::FileKind::parse(&*binary_data)? {
            object::FileKind::Pe32 => Self::read_pe::<ImageNtHeaders32>(&binary_data)?,
            object::FileKind::Pe64 => Self::read_pe::<ImageNtHeaders64>(&binary_data)?,
            _ => Self::read_default(&binary_data)?,
        })
    }

    fn read_default(binary_data: &Vec<u8>) -> Result<Vec<Export>, Box<dyn Error>> {
        let mut exports_result = Vec::<Export>::new();
        let object_file = object::File::parse(&**binary_data)?;

        let exports = object_file.exports()?;
        for export in exports {
            let function_name = str::from_utf8(export.name())?;

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

    fn read_pe<T: ImageNtHeaders>(binary_data: &Vec<u8>) -> Result<Vec<Export>, Box<dyn Error>> {
        let mut exports_result = Vec::<Export>::new();
        let object_file = object::read::pe::PeFile::<T>::parse(&**binary_data)?;

        if let Some(export_table) = object_file.export_table()? {
            for export in export_table.exports()? {
                let function_name = str::from_utf8(export.name.unwrap_or_default())?;

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
}
