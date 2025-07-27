use clap::Parser;
use std::io::ErrorKind;
use crate::args::EntityType;

mod gen_dummy_files;
mod processing;
mod args;

fn main() {
    let args = args::Concurrent_Log_Processor::parse();

    match args.entity_type {
        EntityType::Generate(args) => gen_dummy_file(args.filename, args.lines),
        EntityType::Process(_) => println!("Processing file"),
    }
}

fn gen_dummy_file(filename: Option<String>, lines: Option<u32>) {
    match gen_dummy_files::generate(filename, lines) {
        Ok(()) => println!("Successfully generated dummy file"),
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