//! Module for processing log files with configurable thread support and log level filtering.
//!
//! This module provides functionality to analyze log files, count log levels, and filter logs
//! based on specified criteria. It currently processes logs sequentially but is structured
//! to support concurrent processing in the future.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::enums_and_structs::LogLevel;
use crate::processing::process::process;

/// Processes a log file, filters by specified log level, and provides statistics.
///
/// # Arguments
/// * `threads` - Optional number of threads to use for processing (currently not implemented)
/// * `file` - Path to the log file to process
/// * `log_level` - Optional log level to filter by (defaults to "Error" if not specified)
///
/// # Returns
/// * `std::io::Result<()>` - Ok(()) on success, or an IO error if file operations fail
pub fn spread_tasks(threads: Option<u128>, file: String, log_level: Option<String>) -> std::io::Result<()> {
    // Set default values if not provided
    let threads = threads.unwrap_or(20);
    let loglevel = log_level.unwrap_or("Error".to_string());

    // Initialize counters for different log levels
    let mut log_level_count: u32 = 0;      // Counts lines matching the specified log level
    let mut log_level_counts: HashMap<LogLevel, u128> = HashMap::new(); // Counts occurrences of each log level


    println!("Processing {} with {} threads filtered by {}", file, threads, &loglevel);

    // Open and read the log file line by line
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    // Process each line in the log file
    for line_result in reader.lines() {
        let line = line_result?;
        process(line, &loglevel, &mut log_level_counts, &mut log_level_count);
    }


    // Print summary statistics
    println!("Found {} lines filtered by {}", log_level_count, loglevel);
    println!("Found {} lines filtered by Error", log_level_counts.get(&LogLevel::Error).unwrap_or(&0));
    println!("Found {} lines filtered by Info", log_level_counts.get(&LogLevel::Info).unwrap_or(&0));
    println!("Found {} lines filtered by Debug", log_level_counts.get(&LogLevel::Debug).unwrap_or(&0));
    println!("Found {} lines filtered by Warning", log_level_counts.get(&LogLevel::Warning).unwrap_or(&0));

    // Print total processed lines (sum of all log levels)
    let total_lines = log_level_counts.get(&LogLevel::Error).unwrap_or(&0) + log_level_counts.get(&LogLevel::Info).unwrap_or(&0) + log_level_counts.get(&LogLevel::Debug).unwrap_or(&0) + log_level_counts.get(&LogLevel::Warning).unwrap_or(&0);
    println!("Processed {} lines in total", total_lines);

    Ok(())
}