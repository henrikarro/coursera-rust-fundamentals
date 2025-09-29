use std::fs::File;
use std::io::{Read, Result, Stdin};

#[derive(Debug)]
pub enum Readable {
    File(File),
    Stdin(Stdin),
}

impl Read for Readable {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Readable::File(file) => file.read(buf),
            Readable::Stdin(stdin) => stdin.read(buf),
        }
    }
}

impl Readable {
    pub fn from_file_name(file_name: &str) -> Result<Readable> {
        File::open(file_name).map(Readable::File)
    }

    pub fn from_stdin() -> Readable {
        Readable::Stdin(std::io::stdin())
    }
}
