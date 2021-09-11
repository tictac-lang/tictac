use crate::lexer::lex;
use crate::io::{io_fcreate, io_input, io_output};
use std::path::Path;

pub fn tictac_compile(string: String) {
    let lex = lex(io_input(Path::new(&string).to_path_buf()));
    io_fcreate(Path::new(&format!("{}.lex.cache", string)).to_path_buf());
    io_output(Path::new(&format!("{}.lex.cache", string)).to_path_buf(), format!("{:#?}", lex));
}