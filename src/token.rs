#[derive(Debug, Clone)]
pub enum Token {
    Plus(char),
    Minus(char),
    Equals(char),
    Arer(String),
    Parenth(char),
    String(String),
    Int(String),
    SomethingKeyword(String),
    Whitespace,
    Fault(char),
}