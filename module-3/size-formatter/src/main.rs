#[derive(Debug)]
enum SizeUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

impl SizeUnit {
    fn from_str(s: &str) -> Option<SizeUnit> {
        match s.to_lowercase().as_str() {
            "b" => Some(SizeUnit::Bytes),
            "kb" => Some(SizeUnit::Kilobytes),
            "mb" => Some(SizeUnit::Megabytes),
            "gb" => Some(SizeUnit::Gigabytes),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

impl FileSize {
    fn from_size_and_unit(size: f64, unit: &SizeUnit) -> FileSize {
        match unit {
            SizeUnit::Bytes => FileSize::Bytes(size as u64),
            SizeUnit::Kilobytes => FileSize::Kilobytes(size),
            SizeUnit::Megabytes => FileSize::Megabytes(size),
            SizeUnit::Gigabytes => FileSize::Gigabytes(size),
        }
    }

    fn to_bytes(&self) -> u64 {
        match self {
            FileSize::Bytes(bytes) => *bytes,
            FileSize::Kilobytes(kb) => (kb * 1024.0) as u64,
            FileSize::Megabytes(mb) => (mb * 1_048_576.0) as u64,
            FileSize::Gigabytes(gb) => (gb * 1_073_741_824.0) as u64,
        }
    }

    fn format_size(size: u64) -> String {
        let filesize = match size {
            0..1_024 => FileSize::Bytes(size),
            1_024..1_048_576 => FileSize::Kilobytes(size as f64 / 1024.0),
            1_048_576..1_073_741_824 => FileSize::Megabytes(size as f64 / 1_048_576.0),
            _ => FileSize::Gigabytes(size as f64 / 1_073_741_824.0),
        };
        match filesize {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
            FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (size, unit) = match args.len() {
        2 => (args[1].parse::<f64>(), Some(SizeUnit::Bytes)),
        3 => (args[1].parse::<f64>(), SizeUnit::from_str(&args[2])),
        _ => {
            print_usage_and_exit(&args, 2);
        }
    };
    if let Err(_) = size {
        print_error_and_exit(&format!("Failed to parse {} as a size", args[1]), 1);
    }
    if let None = unit {
        print_error_and_exit(&format!("Unknown size unit: {}", args[2]), 1);
    }
    let file_size = FileSize::from_size_and_unit(size.unwrap(), &unit.unwrap());
    let file_size_in_bytes = file_size.to_bytes();
    println!("{}", FileSize::format_size(file_size_in_bytes));
}

fn print_usage_and_exit(args: &[String], exit_code: i32) -> ! {
    eprintln!("Usage: {} <size> [b|kb|mb|gb]", args[0]);
    std::process::exit(exit_code);
}

fn print_error_and_exit(error: &str, exit_code: i32) -> ! {
    eprintln!("Error: {}", error);
    std::process::exit(exit_code);
}
