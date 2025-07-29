//! Module for processing log files with configurable thread support and log level filtering.
//!
//! This module provides functionality to analyze log files, count log levels, and filter logs
//! based on specified criteria. It currently processes logs sequentially but is structured
//! to support concurrent processing in the future.

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Processes a log file, filters by specified log level, and provides statistics.
///
/// # Arguments
/// * `threads` - Optional number of threads to use for processing (currently not implemented)
/// * `file` - Path to the log file to process
/// * `log_level` - Optional log level to filter by (defaults to "Error" if not specified)
///
/// # Returns
/// * `std::io::Result<()>` - Ok(()) on success, or an IO error if file operations fail
pub fn process(threads: Option<u128>, file: String, log_level: Option<String>) -> std::io::Result<()> {
    // Set default values if not provided
    let threads = threads.unwrap_or(20);
    let loglevel = log_level.unwrap_or("Error".to_string());

    // Initialize counters for different log levels
    let mut log_level_count = 0;      // Counts lines matching the specified log level
    let mut error_log_level_count = 0;   // Counts all Error level logs
    let mut info_log_level_count = 0;    // Counts all Info level logs
    let mut debug_log_level_count = 0;   // Counts all Debug level logs
    let mut warning_log_level_count = 0; // Counts all Warning level logs

    println!("Processing {} with {} threads filtered by {}", file, threads, loglevel);

    // Open and read the log file line by line
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    // Process each line in the log file
    for line_result in reader.lines() {
        let line = line_result?;

        // Check if the line contains the specified log level
        if line.contains(&loglevel) {
            log_level_count += 1;
            println!("{}", line);  // Print lines matching the specified log level
        }

        // Count occurrences of each log level
        if line.contains("Error") {
            error_log_level_count += 1;
        } else if line.contains("Info") {
            info_log_level_count += 1;
        } else if line.contains("Debug") {
            debug_log_level_count += 1;
        } else if line.contains("Warning") {
            warning_log_level_count += 1;
        }
    }

    // Print summary statistics
    println!("Found {} lines filtered by {}", log_level_count, loglevel);
    println!("Found {} lines filtered by Error", error_log_level_count);
    println!("Found {} lines filtered by Info", info_log_level_count);
    println!("Found {} lines filtered by Debug", debug_log_level_count);
    println!("Found {} lines filtered by Warning", warning_log_level_count);

    // Print total processed lines (sum of all log levels)
    let total_lines = error_log_level_count + info_log_level_count + debug_log_level_count + warning_log_level_count;
    println!("Processed {} lines in total", total_lines);

    Ok(())
}