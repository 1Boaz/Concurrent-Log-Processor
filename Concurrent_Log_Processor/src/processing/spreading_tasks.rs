//! Module for processing log files with configurable thread support and log level filtering.
//!
//! This module provides functionality to analyze log files, count log levels, and filter logs
//! based on specified criteria. It currently processes logs sequentially but is structured
//! to support concurrent processing in the future.

use std::collections::HashMap;
use std::io;
use std::sync::atomic;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
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

pub fn spread_tasks(threads: Option<u128>, file: String, log_level: Option<String>) -> io::Result<()> {
    // Default to "Error" if no log level is specified
    let loglevel = log_level.unwrap_or("Error".to_string());
    let num_threads = threads.unwrap_or_else(|| num_cpus::get() as u128);

    println!("Processing {} with {} threads filtered by {}", file, num_threads, &loglevel);

    // Create a thread pool with the specified number of threads
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads as usize)
        .build()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Read the entire file into memory first for better I/O performance
    let file_content = std::fs::read_to_string(&file)?;
    
    // Shared counters
    let log_level_counts = Arc::new(Mutex::new(HashMap::<LogLevel, u128>::new()));
    let log_level_count = Arc::new(atomic::AtomicUsize::new(0));
    
    // Process lines in parallel using the thread pool
    pool.install(|| {
        // Split the file content into lines and process in parallel
        file_content.par_lines()
            .fold(
                || (HashMap::new(), 0),
                |(mut local_counts, mut local_count), line| {
                    process(line.to_string(), &loglevel, &mut local_counts, &mut local_count);
                    (local_counts, local_count)
                }
            )
            .reduce(
                || (HashMap::new(), 0),
                |(mut counts1, total1), (counts2, total2)| {
                    for (k, v) in counts2 {
                        *counts1.entry(k).or_insert(0) += v;
                    }
                    (counts1, total1 + total2)
                }
            )
    });
    
    // Process the file in parallel and get the final counts and total
    let (final_counts, final_total) = file_content.par_lines()
        .fold(
            || (HashMap::new(), 0),
            |(mut local_counts, mut local_count), line| {
                process(line.to_string(), &loglevel, &mut local_counts, &mut local_count);
                (local_counts, local_count)
            }
        )
        .reduce(
            || (HashMap::new(), 0),
            |(mut counts1, total1), (counts2, total2)| {
                for (k, v) in counts2 {
                    *counts1.entry(k).or_insert(0) += v;
                }
                (counts1, total1 + total2)
            }
        );
        
    // Update the shared state with the final results
    let mut counts = log_level_counts.lock().unwrap();
    *counts = final_counts;
    log_level_count.store(final_total as usize, atomic::Ordering::Relaxed);

    // Print summary statistics
    println!("Found {} lines filtered by {}", log_level_count.load(atomic::Ordering::Relaxed), loglevel);
    
    // Reuse the existing lock from above
    println!("Found {} lines filtered by Error", counts.get(&LogLevel::Error).unwrap_or(&0));
    println!("Found {} lines filtered by Info", counts.get(&LogLevel::Info).unwrap_or(&0));
    println!("Found {} lines filtered by Debug", counts.get(&LogLevel::Debug).unwrap_or(&0));
    println!("Found {} lines filtered by Warning", counts.get(&LogLevel::Warning).unwrap_or(&0));

    // Print total processed lines (sum of all log levels)
    let total_lines = counts.values().sum::<u128>();
    println!("Processed {} lines in total", total_lines);
    Ok(())
}