use crate::cmd::diff::*;
use crate::cmd::info::data::*;
use crate::cmd::list::data::*;

use super::PrintOutput;

pub(crate) struct Plain {}

impl PrintOutput for Plain {
    fn print_dependency(&self, dependency: &Dependency) {
        println!("{}", dependency.library);
    }

    fn print_difference(&self, difference: &Difference) {
        let change_data = match difference {
            Difference::Added(data) => {
                print!("+ ");
                data
            }
            Difference::Removed(data) => {
                print!("- ");
                data
            }
        };

        match change_data {
            ChangedData::Dependency(dependency) => self.print_dependency(dependency),
            ChangedData::Export(export) => self.print_export(export),
            ChangedData::Import(import) => self.print_import(import),
        }
    }

    fn print_export(&self, export: &Export) {
        print!(
            "{:#018x}: {} {}",
            export.address.unwrap_or_default(),
            export.function,
            export.function_demangled,
        );

        if let Some(target) = &export.target {
            print!(" -> {}.", target.library);
            match &target.forward {
                ForwardType::Name(name) => print!("{}", name),
                ForwardType::Ordinal(ordinal) => print!("{}", ordinal),
            }
        }

        println!();
    }

    fn print_import(&self, import: &Import) {
        println!(
            "{}: {} {}",
            import.library, import.function, import.function_demangled
        );
    }

    fn print_information(&self, information: &Information) {
        // Basic file information
        println!("Architecture: {:?}", information.architecture);
        println!("Endianess: {:?}", information.endianess);
        println!("Is 64bit: {}", information.is_64);
        println!("ObjectKind: {:?}", information.kind);
        println!("Debug symbols available: {}", information.has_debug_symbols);
        println!(
            "Virtual address of entry point: {:#x}",
            information.entry_address
        );

        // File flags
        // PE/COFF
        if let Some(flags) = information.coff_file_flags {
            println!("Flags: {:?}", flags);
        }

        // PDB infos
        if let Some(pdb) = &information.pdb_info {
            println!();
            println!("PDB:");
            println!("\tAge: {}", pdb.age);
            println!("\tGUID: {}", pdb.guid);
            println!("\tPath: {}", pdb.path);
        }

        // Sections
        println!();
        println!("Sections:");
        for section in &information.sections {
            println!("\tName: {}", section.name);
            println!("\tKind: {:?}", section.kind);
            println!("\tAddress: {:#x}", section.address);
            println!("\tSize: {:#x}", section.size);

            if let Some(segment_name) = &section.segment_name {
                println!("\tSegmentName: {}", segment_name);
            }

            if let Some(flags) = section.coff_section_flags {
                println!("\tFlags: {:?}", flags);
            }
            println!();
        }

        // Symbols
        println!();
        println!("Symbols:");
        for symbol in &information.symbols {
            println!("\tName: {}", symbol.name);
            println!("\tAddress: {:#x}", symbol.address);
            println!("\tSize: {:#x}", symbol.size);
            println!("\tKind: {:?}", symbol.kind);
            println!("\tScope: {:?}", symbol.scope);
            println!("\tSection: {:?}", symbol.section);
            println!();
        }
    }
}
