use std::{
    env,
    io::{BufRead, BufReader},
};

use my_library::readable::Readable;

fn main() {
    let args: Vec<String> = env::args().collect();

    let readable_result = match args.len() {
        1 => Result::Ok(Readable::from_stdin()),
        2 => Readable::from_file_name(&args[1]),
        _ => print_usage_and_exit(&args, 1),
    };

    match readable_result {
        Ok(readable) => {
            copy_lines_to_stdout(BufReader::new(readable));
        }
        Err(error) => {
            print_error_and_exit(error, 2);
        }
    }
}

fn copy_lines_to_stdout(reader: impl BufRead) {
    for line in reader.lines() {
        match line {
            Ok(content) => println!("{}", content),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

fn print_usage_and_exit(args: &[String], exit_code: i32) -> ! {
    eprintln!("Usage: {} [filename]", args[0]);
    std::process::exit(exit_code);
}

fn print_error_and_exit(error: std::io::Error, exit_code: i32) -> ! {
    eprintln!("Error opening file: {}", error);
    std::process::exit(exit_code);
}
