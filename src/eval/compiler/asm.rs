use super::traits::Assembly;
use crate::types::EPathBuf;
use crate::types::EPath;
use crate::types::EString;
use lazy_static::lazy_static;
use std::io::Read;
use std::sync::Mutex;
use std::fs::File;
use std::io::{Error, Write};
use std::fs::OpenOptions;

lazy_static!{
    #[derive(Debug)]
    pub static ref ASMInstance: Mutex<ASM> = Mutex::new(ASM::create_struct(String::from("./tictac.s")));
}

#[derive(Debug)]
pub struct ASM {
    pub path: EPathBuf
}

impl Assembly for ASM {
    fn write_out(&mut self, template: crate::types::ASMTemplate) -> Result<(), Error> {
        let path = self.path.as_path();
        let mut output = OpenOptions::new().read(true).write(true).append(true).open(path).unwrap();
        Ok(
        for x in template {
            writeln!(output, "{0}", x).unwrap();
            //output.write_all(format!("{}\n", x).as_bytes()).unwrap();
        }
    )
    }

    fn create_out(&mut self) {
        let _nfile = File::create(self.path.as_path());
    }

    fn create_struct(path: EString) -> Self {
        Self {
            path: EPath::new(&path).to_path_buf()
        }
    }
}