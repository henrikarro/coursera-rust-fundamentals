#[derive(Debug, Clone)]
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

    fn multiplier(&self) -> f64 {
        match self {
            SizeUnit::Bytes => 1.0,
            SizeUnit::Kilobytes => 1024.0,
            SizeUnit::Megabytes => 1_048_576.0,
            SizeUnit::Gigabytes => 1_073_741_824.0,
        }
    }

    fn to_bytes(&self, size: f64) -> u64 {
        (size * self.multiplier()) as u64
    }

    fn to_string(&self) -> &str {
        match self {
            SizeUnit::Bytes => "bytes",
            SizeUnit::Kilobytes => "KB",
            SizeUnit::Megabytes => "MB",
            SizeUnit::Gigabytes => "GB",
        }
    }
}

#[derive(Debug)]
struct FileSize {
    size: f64,
    unit: SizeUnit,
}

impl FileSize {
    fn from_size_and_unit(size: f64, unit: &SizeUnit) -> FileSize {
        FileSize {
            size: size,
            unit: unit.clone(),
        }
    }

    fn from_size_in_bytes(size: u64) -> FileSize {
        match size {
            0..1_024 => FileSize::from_size_and_unit(size as f64, &SizeUnit::Bytes),
            1_024..1_048_576 => FileSize::from_size_and_unit(size as f64 / SizeUnit::Kilobytes.multiplier(), &SizeUnit::Kilobytes),
            1_048_576..1_073_741_824 => FileSize::from_size_and_unit(size as f64 / SizeUnit::Megabytes.multiplier(), &SizeUnit::Megabytes),
            _ => FileSize::from_size_and_unit(size as f64 / SizeUnit::Gigabytes.multiplier(), &SizeUnit::Gigabytes),
        }
    }

    fn to_unit(&self, unit: &SizeUnit) -> FileSize {
        let size_in_bytes = self.to_bytes();
        FileSize::from_size_and_unit(size_in_bytes as f64 / unit.multiplier(), unit)
    }

    fn to_bytes(&self) -> u64 {
        self.unit.to_bytes(self.size)
    }

    fn to_string(&self) -> String {
        format!("{:.2} {}", self.size, self.unit.to_string())
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
    let size = size.unwrap();
    let unit = unit.unwrap();
    let file_size = FileSize::from_size_in_bytes(unit.to_bytes(size));
    println!(
        "{} ({}, {}, {}, {})",
        file_size.to_string(),
        file_size.to_unit(&SizeUnit::Bytes).to_string(),
        file_size.to_unit(&SizeUnit::Kilobytes).to_string(),
        file_size.to_unit(&SizeUnit::Megabytes).to_string(),
        file_size.to_unit(&SizeUnit::Gigabytes).to_string()
    );
}

fn print_usage_and_exit(args: &[String], exit_code: i32) -> ! {
    eprintln!("Usage: {} <size> [b|kb|mb|gb]", args[0]);
    std::process::exit(exit_code);
}

fn print_error_and_exit(error: &str, exit_code: i32) -> ! {
    eprintln!("Error: {}", error);
    std::process::exit(exit_code);
}
