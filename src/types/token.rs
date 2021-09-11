use super::EArrayToken;

#[derive(Debug, Clone)]
pub enum Token {
    Semicolon(char),
    Plus(char),
    Minus(char),
    Equals(char),
    Arer(String),
    Parenth(char),
    Pipe(char),
    Braces(EArrayToken),
    RBraces,
    Comma,
    String(String),
    Int(String),
    SomethingKeyword(String),
    Keyword(Keyword),
    SomeName(String),
    FunctionDefinition(String),
    FunctionCall(String),
    SomethingFunction(String),
    NewLine,
    SpecialNL,
    Whitespace,
    Fault(String, String),
    EMBreak
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Maybe,
    Exports,
    This
}