#![feature(exclusive_range_pattern)]
mod lexer;
mod generalize;
pub mod token;
pub mod eval;
pub mod types;
use eval::compiler::traits::Assembly;

fn main() {
    let togen = lexer::lex(String::from("-\"test\"->+- maybe 0123 hty 398587 ash"));
    let gen = generalize::rules(togen.clone());
    debug!("{:#?}", gen);
    debug!("Assembler Testing:");
}

#[macro_export]
macro_rules! debug {
    ( $($arg:tt)* ) => {
        println!("[Debug] {}", format_args!($($arg)*));
    };
}