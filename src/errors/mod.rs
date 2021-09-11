#[derive(Debug)]
pub enum Error {
    NoSemicolon,
    UnknownChar,
    EndBracesAdvanceCorruption
}

pub fn error(error: Error, line: usize, chars: char) {
    println!("Error at line {1}: {0:?}\nChar: {2}\n\n", error, line, chars);
    panic!("{:?}", error)
}