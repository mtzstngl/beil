pub struct Dependency {
    pub library: String,
}

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

pub struct Import {
    pub library: String,
    pub function: String,
    pub function_demangled: String,
}
