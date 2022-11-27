use std::{fs, path::PathBuf};

use clap::Args;
use object::{FileFlags, Object, ObjectSection, ObjectSymbol, SectionFlags};
use uuid::Uuid;

use crate::output::PrintOutput;

use self::data::*;

pub mod data;

// Command line arguments for the info module.
#[derive(Args)]
pub struct Arguments {
    /// The file of which to display information.
    file: PathBuf,
}

pub fn run(arguments: &Arguments, output: &dyn PrintOutput) {
    let binary_data = fs::read(&arguments.file).unwrap();
    let object_file = object::File::parse(&*binary_data).unwrap();

    // File flags for PE/COFF
    let flags = if let FileFlags::Coff { characteristics: c } = object_file.flags() {
        CoffFileFlags::from_bits(c)
    } else {
        None
    };

    // PDB infos
    let pdb = if let Ok(Some(pdb_info)) = object_file.pdb_info() {
        Some(PdbInfo {
            age: pdb_info.age(),
            guid: Uuid::from_bytes(pdb_info.guid()).hyphenated().to_string(),
            path: String::from_utf8_lossy(pdb_info.path()).into(),
        })
    } else {
        None
    };

    // Sections
    let mut sections = Vec::<Section>::new();
    for section in object_file.sections() {
        let flags = if let SectionFlags::Coff { characteristics: c } = section.flags() {
            CoffSectionFlags::from_bits(c)
        } else {
            None
        };

        let segment_name = if let Ok(Some(name)) = section.segment_name() {
            Some(name.to_string())
        } else {
            None
        };

        sections.push(Section {
            address: section.address(),
            name: section.name().unwrap_or_default().to_string(),
            kind: section.kind(),
            size: section.size(),
            segment_name,
            coff_section_flags: flags,
        });
    }

    // Symbols
    let mut symbols = Vec::<Symbol>::new();
    for symbol in object_file.symbols() {
        symbols.push(Symbol {
            address: symbol.address(),
            kind: symbol.kind(),
            name: symbol.name().unwrap_or_default().to_owned(),
            scope: symbol.scope(),
            section: symbol.section(),
            size: symbol.size(),
        });
    }

    output.print_information(&Information {
        architecture: object_file.architecture(),
        endianess: object_file.endianness(),
        is_64: object_file.is_64(),
        kind: object_file.kind(),
        has_debug_symbols: object_file.has_debug_symbols(),
        entry_address: object_file.entry(),
        coff_file_flags: flags,
        pdb_info: pdb,
        sections,
        symbols,
    });
}
