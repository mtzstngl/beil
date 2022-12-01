use std::{error::Error, fs, path::Path, str};

use object::Object;
use symbolic::{
    common::Name,
    demangle::{Demangle, DemangleOptions},
};

pub struct Import {
    pub library: String,
    pub function: String,
    pub function_demangled: String,
}

impl Import {
    pub fn read(file: &Path) -> Result<Vec<Import>, Box<dyn Error>> {
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
}
