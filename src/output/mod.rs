use clap::ValueEnum;

use crate::cmd::diff::*;
use crate::cmd::info::data::*;
use crate::cmd::list::data::*;

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
