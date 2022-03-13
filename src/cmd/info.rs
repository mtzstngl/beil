use bitflags::bitflags;
use clap::Args;
use object::{pe, FileFlags, Object, ObjectSection, ObjectSymbol, SectionFlags};
use std::{fs, path::PathBuf, str};
use uuid::Uuid;

// Command line arguments for the info module.
#[derive(Args)]
pub struct Arguments {
    /// The file of which to display information.
    file: PathBuf,
}

// use bitflags! to make it easier to print the set file flags.
bitflags! {
    struct CoffFileFlags : u16 {
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
    struct CoffSectionFlags : u32 {
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

pub fn run(arguments: &Arguments) {
    let binary_data = fs::read(&arguments.file).unwrap();
    let object_file = object::File::parse(&*binary_data).unwrap();

    // Basic file information
    println!("Architecture: {:?}", object_file.architecture());
    println!("Endianess: {:?}", object_file.endianness());
    println!("Is 64bit: {}", object_file.is_64());
    println!("ObjectKind: {:?}", object_file.kind());
    println!(
        "Debug symbols available: {}",
        object_file.has_debug_symbols()
    );
    println!("Virtual address of entry point: {:#x}", object_file.entry());

    // File flags
    // PE/COFF
    if let FileFlags::Coff { characteristics: c } = object_file.flags() {
        if let Some(flags) = CoffFileFlags::from_bits(c) {
            println!("Flags: {:?}", flags);
        }
    }

    // PDB infos
    if let Ok(Some(pdb)) = object_file.pdb_info() {
        println!();
        println!(
            "PDB:\n    Age: {}\n    GUID: {}\n    Path: {}",
            pdb.age(),
            Uuid::from_bytes(pdb.guid()).to_hyphenated(),
            str::from_utf8(pdb.path()).unwrap_or_default()
        );
    }

    // Sections
    println!();
    println!("Sections:");
    for section in object_file.sections() {
        println!("    Name: {}", section.name().unwrap_or_default());
        println!("    Kind: {:?}", section.kind());
        println!("    Address: {:#x}", section.address());
        println!("    Size: {:#x}", section.size());
        if let Ok(Some(segment_name)) = section.segment_name() {
            println!("    SegmentName: {}", segment_name);
        }

        if let SectionFlags::Coff { characteristics: c } = section.flags() {
            if let Some(flags) = CoffSectionFlags::from_bits(c) {
                println!("    Flags: {:?}", flags);
            }
        }
        println!();
    }

    // Symbols
    println!();
    println!("Symbols:");
    for symbol in object_file.symbols() {
        println!("    Name: {}", symbol.name().unwrap_or_default());
        println!("    Address: {:#x}", symbol.address());
        println!("    Size: {:#x}", symbol.size());
        println!("    Kind: {:?}", symbol.kind());
        println!("    Scope: {:?}", symbol.scope());
        println!("    Section: {:?}", symbol.section());
        println!();
    }
}
