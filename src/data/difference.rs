use super::dependency::Dependency;
use super::export::Export;
use super::import::Import;

pub enum Difference {
    Added(ChangedData),
    Removed(ChangedData),
}

pub enum ChangedData {
    Dependency(Dependency),
    Export(Export),
    Import(Import),
}
