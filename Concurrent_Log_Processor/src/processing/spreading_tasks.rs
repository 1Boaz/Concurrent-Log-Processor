use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process(threads: Option<u128>, file: String, log_level: Option<String>) -> std::io::Result<()> {
    let threads = threads.unwrap_or(20);
    let loglevel = log_level.unwrap_or("Error".to_string());
    let mut log_level_count = 0;
    let mut error_log_level_count = 0;
    let mut info_log_level_count = 0;
    let mut debug_log_level_count = 0;
    let mut warning_log_level_count = 0;

    println!("Processing {} with {} threads filtered by {}", file, threads, loglevel);

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(&loglevel) {
            log_level_count += 1;
            println!("{}", line);
        } else if line.contains("Error") {
            error_log_level_count += 1;
        } else if line.contains("Info") {
            info_log_level_count += 1;
        } else if line.contains("Debug") {
            debug_log_level_count += 1;
        } else if line.contains("Warning") {
            warning_log_level_count += 1;
        }
    }

    println!("Found {} lines filtered by {}", log_level_count, loglevel);

    println!("Found {} lines filtered by Error", error_log_level_count);
    println!("Found {} lines filtered by Info", info_log_level_count);
    println!("Found {} lines filtered by Debug", debug_log_level_count);
    println!("Found {} lines filtered by Warning", warning_log_level_count);

    Ok(())
}