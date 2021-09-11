#[derive(Debug)]
pub enum Error {
    NoSemicolon,
    UnknownChar,
    EndBracesAdvanceCorruption
}

pub fn error(error: Error, line: usize) {
    println!("Error at line {1}: {0:?}\n\n", error, line);
    panic!("{:?}", error)
}