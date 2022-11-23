use super::PrintOutput;

pub(crate) struct Plain {}

impl PrintOutput for Plain {
    fn print_dependency(&self, dependency: &super::Dependency) {}

    fn print_export(&self, export: &super::Export) {}

    fn print_import(&self, import: &super::Import) {}

    fn print_information(&self, information: &super::Information) {}
}
