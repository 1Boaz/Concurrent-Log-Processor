use std::io::ErrorKind;

mod gen_dummy_files;

fn main() {
    match gen_dummy_files::generate() {
        Ok(()) => println!("Successfully generated dummy files"),
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => println!("Failed to generate dummy file, lacks permission to create or write to file: {}", error),
            ErrorKind::NotFound => println!("Path to file not found: {}", error),
            ErrorKind::AlreadyExists=> println!("The File already exists and could not be appended to or overeaten: {}", error),
            ErrorKind::TimedOut => println!("Timed out waiting for file creation: {}", error),
            ErrorKind::Interrupted => println!("Interrupted by user input: {}", error),
            ErrorKind::StorageFull => println!("The disk is full and can`t save the file: {}", error),
            ErrorKind::InvalidInput | ErrorKind::InvalidData => println!("Invalid path to file: {}", error),
            ErrorKind::WriteZero => println!("Failed to write to file: {}", error),
            _ => println!("Failed to generate dummy file: {}", error),
        }
    }
}
