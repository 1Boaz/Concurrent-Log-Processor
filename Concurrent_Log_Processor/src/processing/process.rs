use std::collections::HashMap;
use crate::enums_and_structs::LogLevel;

pub fn process(line: String, loglevel: &String, log_level_counts: &mut HashMap<LogLevel, u128>, log_level_count: &mut u32) {
    // Check if the line contains the specified log level
    if line.contains(loglevel) {
        *log_level_count += 1;
        println!("{}", line);  // Print lines matching the specified log level
    }
    check_log_levels(line, log_level_counts)
}

fn check_log_levels(line: String, log_level_counts: &mut HashMap<LogLevel, u128>) {
    if line.contains("Error") {
        *log_level_counts.entry(LogLevel::Error).or_insert(0) += 1;
    } else if line.contains("Info") {
        *log_level_counts.entry(LogLevel::Info).or_insert(0) += 1;
    } else if line.contains("Debug") {
        *log_level_counts.entry(LogLevel::Debug).or_insert(0) += 1;
    } else if line.contains("Warning") {
        *log_level_counts.entry(LogLevel::Warning).or_insert(0) += 1;
    }
}