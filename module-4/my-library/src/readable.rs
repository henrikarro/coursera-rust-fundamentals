//! A library for reading from either a file or standard input.
//! # Examples
//! ```
//! use std::io::Read;
//! use my_library::readable::Readable;
//! let mut readable = Readable::from_file_name("Cargo.toml").unwrap();
//! let mut contents = String::new();
//! readable.read_to_string(&mut contents).unwrap();
//! println!("{}", contents);
//! ```

use std::fs::File;
use std::io::{Read, Result, Stdin};

/// An enum that can represent either a file or standard input.
#[derive(Debug)]
pub enum Readable {
    /// A file.
    File(File),

    /// Standard input.
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
    /// Create a `Readable` from a file name.
    /// If the file cannot be opened, an error is returned.
    /// # Examples
    /// ```
    /// use std::io::Read;
    /// use my_library::readable::Readable;
    /// let mut readable = Readable::from_file_name("Cargo.toml").unwrap();
    /// let mut contents = String::new();
    /// readable.read_to_string(&mut contents).unwrap();
    /// println!("{}", contents);
    /// ```
    pub fn from_file_name(file_name: &str) -> Result<Readable> {
        File::open(file_name).map(Readable::File)
    }

    /// Create a `Readable` from standard input.
    pub fn from_stdin() -> Readable {
        Readable::Stdin(std::io::stdin())
    }
}
