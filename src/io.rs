// used for file reading
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::fs::File;

pub fn io_input(path: PathBuf) -> String {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

pub fn io_output(path: PathBuf, output: String) {
    let mut file = OpenOptions::new().read(true).write(true).append(true).open(path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

pub fn io_fcreate(path: PathBuf) {
    let mut _file = File::create(path);
}