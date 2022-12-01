use bitflags::bitflags;
use object::{
    pe, Architecture, Endianness, ObjectKind, SectionKind, SymbolKind, SymbolScope, SymbolSection,
};

use std::{error::Error, fs, path::Path};

use object::{FileFlags, Object, ObjectSection, ObjectSymbol, SectionFlags};
use uuid::Uuid;

// use bitflags! to make it easier to print the set file flags.
bitflags! {
    pub struct CoffFileFlags : u16 {
        // Image only, Windows CE, and Microsoft Windows NT and later.
        // This indicates that the file does not contain base relocations and
        // must therefore be loaded at its preferred base address.
        // If the base address is not available, the loader reports an error.
        // The default behavior of the linker is to strip base relocations from
        // executable (EXE) files.
        const RELOCS_STRIPPED = pe::IMAGE_FILE_RELOCS_STRIPPED;
        // Image only.
        // This indicates that the image file is valid and can be run.
        // If this flag is not set, it indicates a linker error.
        const EXECUTABLE_IMAGE = pe::IMAGE_FILE_EXECUTABLE_IMAGE;
        // COFF line numbers have been removed.
        // This flag is deprecated and should be zero.
        const LINE_NUMS_STRIPPED = pe::IMAGE_FILE_LINE_NUMS_STRIPPED;
        // COFF symbol table entries for local symbols have been removed.
        // This flag is deprecated and should be zero.
        const LOCAL_SYMS_STRIPPED = pe::IMAGE_FILE_LOCAL_SYMS_STRIPPED;
        // Obsolete. Aggressively trim working set.
        // This flag is deprecated for Windows 2000 and later and must be zero.
        const AGGRESSIVE_WS_TRIM = pe::IMAGE_FILE_AGGRESIVE_WS_TRIM;
        // Application can handle > 2-GB addresses.
        const LARGE_ADDRESS_AWARE = pe::IMAGE_FILE_LARGE_ADDRESS_AWARE;
        // This flag is reserved for future use.
        const RESERVED = 0x0040;
        // Little endian: the least significant bit (LSB) precedes the most
        // significant bit (MSB) in memory.
        // This flag is deprecated and should be zero.
        const BYTES_REVERSED_LO = pe::IMAGE_FILE_BYTES_REVERSED_LO;
        // Machine is based on a 32-bit-word architecture.
        const IS32BIT_MACHINE = pe::IMAGE_FILE_32BIT_MACHINE;
        // Debugging information is removed from the image file.
        const DEBUG_STRIPPED = pe::IMAGE_FILE_DEBUG_STRIPPED;
        // If the image is on removable media, fully load it and copy it to the
        // swap file.
        const REMOVABLE_RUN_FROM_SWAP = pe::IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP;
        // If the image is on network media, fully load it and copy it to the
        // swap file.
        const NET_RUN_FROM_SWAP = pe::IMAGE_FILE_NET_RUN_FROM_SWAP;
        // The image file is a system file, not a user program.
        const SYSTEM = pe::IMAGE_FILE_SYSTEM;
        // The image file is a dynamic-link library (DLL).
        // Such files are considered executable files for almost all purposes,
        // although they cannot be directly run.
        const DLL = pe::IMAGE_FILE_DLL;
        // The file should be run only on a uniprocessor machine.
        const UP_SYSTEM_ONLY = pe::IMAGE_FILE_UP_SYSTEM_ONLY;
        // Big endian: the MSB precedes the LSB in memory.
        // This flag is deprecated and should be zero.
        const BYTES_REVERSED_HI = pe::IMAGE_FILE_BYTES_REVERSED_HI;
    }
}

bitflags! {
    pub struct CoffSectionFlags : u32 {
        // Reserved for future use.
        const RESERVED1 = 0x0000_0000;
        // Reserved for future use.
        const RESERVED2 = 0x0000_0001;
        // Reserved for future use.
        const RESERVED3 = 0x0000_0002;
        // Reserved for future use.
        const RESERVED4 = 0x0000_0004;
        // The section should not be padded to the next boundary.
        // This flag is obsolete and is replaced by IMAGE_SCN_ALIGN_1BYTES.
        // This is valid only for object files.
        const TYPE_NO_PAD = pe::IMAGE_SCN_TYPE_NO_PAD;
        // Reserved for future use.
        const RESERVED5 = 0x0000_0010;
        // The section contains executable code.
        const CNT_CODE = pe::IMAGE_SCN_CNT_CODE;
        // The section contains initialized data.
        const CNT_INITIALIZED_DATA = pe::IMAGE_SCN_CNT_INITIALIZED_DATA;
        // The section contains uninitialized data.
        const CNT_UNINITIALIZED_DATA = pe::IMAGE_SCN_CNT_UNINITIALIZED_DATA;
        // Reserved for future use.
        const LNK_OTHER = pe::IMAGE_SCN_LNK_OTHER;
        // The section contains comments or other information.
        // The .drectve section has this type.
        // This is valid for object files only.
        const LNK_INFO = pe::IMAGE_SCN_LNK_INFO;
        // Reserved for future use.
        const RESERVED6 = 0x0000_0400;
        // The section will not become part of the image.
        // This is valid only for object files.
        const LNK_REMOVE = pe::IMAGE_SCN_LNK_REMOVE;
        // The section contains COMDAT data.
        // For more information, see COMDAT Sections (Object Only).
        // This is valid only for object files.
        const LNK_COMDAT = pe::IMAGE_SCN_LNK_COMDAT;
        // The section contains data referenced through the global pointer (GP).
        const GPREL = pe::IMAGE_SCN_GPREL;
        // Reserved for future use.
        const MEM_PURGEABLE = pe::IMAGE_SCN_MEM_PURGEABLE;
        // Reserved for future use.
        const MEM_16BIT = pe::IMAGE_SCN_MEM_16BIT;
        // Reserved for future use.
        const MEM_LOCKED = pe::IMAGE_SCN_MEM_LOCKED;
        // Reserved for future use.
        const MEM_PRELOAD = pe::IMAGE_SCN_MEM_PRELOAD;
        // Align data on a 1-byte boundary. Valid only for object files.
        const ALIGN_1BYTES = pe::IMAGE_SCN_ALIGN_1BYTES;
        // Align data on a 2-byte boundary. Valid only for object files.
        const ALIGN_2BYTES = pe::IMAGE_SCN_ALIGN_2BYTES;
        // Align data on a 4-byte boundary. Valid only for object files.
        const ALIGN_4BYTES = pe::IMAGE_SCN_ALIGN_4BYTES;
        // Align data on an 8-byte boundary. Valid only for object files.
        const ALIGN_8BYTES = pe::IMAGE_SCN_ALIGN_8BYTES;
        // Align data on a 16-byte boundary. Valid only for object files.
        const ALIGN_16BYTES = pe::IMAGE_SCN_ALIGN_16BYTES;
        // Align data on a 32-byte boundary. Valid only for object files.
        const ALIGN_32BYTES = pe::IMAGE_SCN_ALIGN_32BYTES;
        // Align data on a 64-byte boundary. Valid only for object files.
        const ALIGN_64BYTES = pe::IMAGE_SCN_ALIGN_64BYTES;
        // Align data on a 128-byte boundary. Valid only for object files.
        const ALIGN_128BYTES = pe::IMAGE_SCN_ALIGN_128BYTES;
        // Align data on a 256-byte boundary. Valid only for object files.
        const ALIGN_256BYTES = pe::IMAGE_SCN_ALIGN_256BYTES;
        // Align data on a 512-byte boundary. Valid only for object files.
        const ALIGN_512BYTES = pe::IMAGE_SCN_ALIGN_512BYTES;
        // Align data on a 1024-byte boundary. Valid only for object files.
        const ALIGN_1024BYTES = pe::IMAGE_SCN_ALIGN_1024BYTES;
        // Align data on a 2048-byte boundary. Valid only for object files.
        const ALIGN_2048BYTES = pe::IMAGE_SCN_ALIGN_2048BYTES;
        // Align data on a 4096-byte boundary. Valid only for object files.
        const ALIGN_4096BYTES = pe::IMAGE_SCN_ALIGN_4096BYTES;
        // Align data on an 8192-byte boundary. Valid only for object files.
        const ALIGN_8192BYTES = pe::IMAGE_SCN_ALIGN_8192BYTES;
        // The section contains extended relocations.
        const LNK_NRELOC_OVFL = pe::IMAGE_SCN_LNK_NRELOC_OVFL;
        // The section can be discarded as needed.
        const MEM_DISCARDABLE = pe::IMAGE_SCN_MEM_DISCARDABLE;
        // The section cannot be cached.
        const MEM_NOT_CACHED = pe::IMAGE_SCN_MEM_NOT_CACHED;
        // The section is not pageable.
        const MEM_NOT_PAGED = pe::IMAGE_SCN_MEM_NOT_PAGED;
        // The section can be shared in memory.
        const MEM_SHARED = pe::IMAGE_SCN_MEM_SHARED;
        // The section can be executed as code.
        const MEM_EXECUTE = pe::IMAGE_SCN_MEM_EXECUTE;
        // The section can be read.
        const MEM_READ = pe::IMAGE_SCN_MEM_READ;
        // The section can be written to.
        const MEM_WRITE = pe::IMAGE_SCN_MEM_WRITE;
    }
}

pub struct Information {
    pub architecture: Architecture,
    pub endianess: Endianness,
    pub is_64: bool,
    pub kind: ObjectKind,
    pub has_debug_symbols: bool,
    pub entry_address: u64,
    pub coff_file_flags: Option<CoffFileFlags>,
    pub pdb_info: Option<PdbInfo>,
    pub sections: Vec<Section>,
    pub symbols: Vec<Symbol>,
}

pub struct PdbInfo {
    pub age: u32,
    pub guid: String,
    pub path: String,
}

pub struct Section {
    pub name: String,
    pub kind: SectionKind,
    pub address: u64,
    pub size: u64,
    pub segment_name: Option<String>,
    pub coff_section_flags: Option<CoffSectionFlags>,
}

pub struct Symbol {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub kind: SymbolKind,
    pub scope: SymbolScope,
    pub section: SymbolSection,
}

impl Information {
    pub fn read(file: &Path) -> Result<Self, Box<dyn Error>> {
        let binary_data = fs::read(&file)?;
        let object_file = object::File::parse(&*binary_data)?;

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

        Ok(Information {
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
        })
    }
}
