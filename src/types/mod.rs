use self::token::Token;

pub mod token;

pub type EString = String;
pub type ASMTemplate = Vec<EString>;
pub type EPath = std::path::Path;
pub type EPathBuf = std::path::PathBuf;
pub type EInt = usize;
pub type EArrayString = Vec<EString>;
pub type EArrayToken = Vec<Token>;