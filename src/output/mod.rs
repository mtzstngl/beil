use clap::ValueEnum;

use crate::data::{
    dependency::Dependency, difference::Difference, export::Export, import::Import,
    info::Information,
};

pub mod plain;

#[derive(ValueEnum, Copy, Clone)]
pub enum OutputType {
    /// Outputs everything as plain text.
    Plain,
}

impl OutputType {
    pub fn to_output(self) -> Box<dyn PrintOutput> {
        match self {
            OutputType::Plain => Box::new(plain::Plain {}),
        }
    }
}

pub trait PrintOutput {
    fn print_dependency(&self, dependency: &Dependency);
    fn print_difference(&self, difference: &Difference);
    fn print_export(&self, export: &Export);
    fn print_import(&self, import: &Import);
    fn print_information(&self, information: &Information);
}
