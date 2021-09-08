use crate::types::ASMTemplate;
use crate::types::EPath;
use crate::types::EString;
use std::fs::File;
use std::io::{Error, Write};

// trait for writing assembly to output file
pub trait Assembly {
    fn write_out(&mut self, template: ASMTemplate) -> Result<(), Error>;

    fn create_out(&mut self);

    fn create_struct(path: EString) -> Self;
}