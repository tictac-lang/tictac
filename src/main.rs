#![feature(exclusive_range_pattern)]
use std::env;
mod lexer;
mod parser;
pub mod eval;
pub mod types;
pub mod errors;
pub mod tictac;
pub mod io;
use eval::compiler::traits::Assembly;

fn main() {
    let args: Vec<String> = env::args().collect();
    tictac::tictac_compile(args[1].clone())
    /*
    let togen = lexer::lex(String::from("(test) -> {\"afet\", 123}"));
    debug!("{:#?}", togen);
    debug!("Assembler Testing:");
    */
}

#[macro_export]
macro_rules! debug {
    ( $($arg:tt)* ) => {
        println!("[Debug] {}", format_args!($($arg)*));
    };
}