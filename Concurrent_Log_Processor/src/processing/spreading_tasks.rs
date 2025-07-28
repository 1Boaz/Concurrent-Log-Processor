pub fn process(threads: Option<u128>, file: String, log_level: Option<String>) -> std::io::Result<()> {
    let threads = threads.unwrap_or(20);
    let loglevel = log_level.unwrap_or("".to_string());

    println!("Processing {} with {} threads filtered by {}", file, threads, loglevel);
    Ok(())
}