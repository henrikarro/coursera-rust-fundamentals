//! A simple file operations library.
//!
//! # Examples
//! ```
//! use file_utils::{read_file, write_file, append_to_file};
//!
//! // Write to a file
//! write_file("example.txt", "Hello, world!\n").unwrap();
//!
//! // Read from a file
//! let content = read_file("example.txt").unwrap();
//! assert_eq!(content, "Hello, world!\n");
//!
//! // Append to a file
//! append_to_file("example.txt", "This is an appended line.").unwrap();
//! let updated_content = read_file("example.txt").unwrap();
//! assert_eq!(updated_content, "Hello, world!\nThis is an appended line.\n");
//! ```
//! # Errors
//! All functions return a `std::io::Result` which will contain an error if the
//! operation fails.

/// Reads the entire contents of a file into a string.
/// # Arguments
/// * `file_name` - The name of the file to read.
/// # Returns
/// A `Result` which is `Ok` if the operation was successful, containing the file
/// contents as a `String`, or an `Err` if an error occurred.
///
pub fn read_file(file_name: &str) -> std::io::Result<String> {
    std::fs::read_to_string(file_name)
}

/// Writes a string to a file, overwriting the file if it already exists.
/// # Arguments
/// * `file_name` - The name of the file to write to.
/// * `content` - The content to write to the file.
/// # Returns
/// A `Result` which is `Ok` if the operation was successful, or an `Err` if an error occurred.
///
pub fn write_file(file_name: &str, content: &str) -> std::io::Result<()> {
    std::fs::write(file_name, content)
}

/// Appends a string to the end of a file, creating the file if it does not exist.
/// # Arguments
/// * `file_name` - The name of the file to append to.
/// * `content` - The content to append to the file.
/// # Returns
/// A `Result` which is `Ok` if the operation was successful, or an `Err` if an error occurred.
///
pub fn append_to_file(file_name: &str, content: &str) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)?;
    writeln!(file, "{}", content)?;
    Ok(())
}