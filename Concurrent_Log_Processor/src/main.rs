//! Concurrent Log Processor
//!
//! A command-line utility for generating and processing log files with configurable concurrency.
//!
//! This program provides two main functionalities:
//! 1. Generate dummy log files for testing purposes
//! 2. Process log files with configurable thread support and log level filtering

use clap::Parser;
use std::io::ErrorKind;
use args::EntityType;
use crate::args::ProcessCommand;

mod gen_dummy_files;
mod processing;
mod args;

/// Main entry point of the application.
///
/// Parses command line arguments and routes to the appropriate functionality
/// based on the subcommand provided (generate or process).
fn main() {
    // Start timing the execution
    let start = std::time::Instant::now();

    // Parse command line arguments using clap
    let args = args::Concurrent_Log_Processor::parse();

    // Match the subcommand and execute the corresponding functionality
    match args.entity_type {
        EntityType::Generate(args) => gen_dummy_file(args.filename, args.lines),
        EntityType::Process(args) => process_file(args),
    }

    // Calculate and print the total execution time
    let duration = start.elapsed();
    println!("Finished in {} ms", duration.as_millis());
}

/// Generates a dummy log file with random log entries.
///
/// # Arguments
/// * `filename` - Optional name for the output file
/// * `lines` - Optional number of log lines to generate
fn gen_dummy_file(filename: Option<String>, lines: Option<u32>) {
    match gen_dummy_files::generate(filename, lines) {
        Ok(()) => println!("Successfully generated dummy file"),
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => println!("Failed to generate dummy file, lacks permission to create or write to file: {}", error),
            ErrorKind::NotFound => println!("Path to file not found: {}", error),
            ErrorKind::AlreadyExists=> println!("The File already exists and could not be appended to or overwritten: {}", error),
            ErrorKind::TimedOut => println!("Timed out waiting for file creation: {}", error),
            ErrorKind::Interrupted => println!("Interrupted by user input: {}", error),
            ErrorKind::StorageFull => println!("The disk is full and can`t save the file: {}", error),
            ErrorKind::InvalidInput | ErrorKind::InvalidData => println!("Invalid path to file: {}", error),
            ErrorKind::WriteZero => println!("Failed to write to file: {}", error),
            _ => println!("Failed to generate dummy file: {}", error),
        }
    }
}

/// Processes a log file with the specified parameters.
///
/// # Arguments
/// * `args` - ProcessCommand struct containing processing parameters:
///   - threads: Number of threads to use for processing
///   - file: Path to the log file to process
///   - log_level: Log level to filter by
fn process_file(args: ProcessCommand) {
    match processing::spreading_tasks::process(args.threads, args.file, args.log_level) {
        Ok(()) => {
            println!("Successfully processed file");
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::PermissionDenied => println!("Failed to process file, lacks permission to create or write to file: {}", error),
                ErrorKind::NotFound => println!("Path to file not found: {}", error),
                ErrorKind::AlreadyExists=> println!("The File already exists and could not be appended to or overwritten: {}", error),
                ErrorKind::TimedOut => println!("Timed out waiting for file creation: {}", error),
                ErrorKind::Interrupted => println!("Interrupted by user input: {}", error),
                ErrorKind::StorageFull => println!("The disk is full and can`t save the file: {}", error),
                ErrorKind::InvalidInput | ErrorKind::InvalidData => println!("Invalid path to file: {}", error),
                ErrorKind::WriteZero => println!("Failed to write to file: {}", error),
                _ => println!("Failed to process file: {}", error),
            }
        },
    }
}