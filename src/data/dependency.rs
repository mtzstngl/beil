use std::{error::Error, fs, path::Path, str};

use object::Object;

pub struct Dependency {
    pub library: String,
}

impl Dependency {
    pub fn read(file: &Path) -> Result<Vec<Dependency>, Box<dyn Error>> {
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
}
