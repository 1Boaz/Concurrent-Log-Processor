use std::io::{BufWriter, Write};
use std::fs::File;
use rand::prelude::*;
use rayon::prelude::*;

#[derive(Debug)]
enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Debug)]
struct Timestamp {
    hour: u8,
    minute: u8,
    second: u8
}

impl std::fmt::Display for Timestamp {
    // Formats the `Timestamp` struct for display.
    //
    // This implementation of the `fmt` method formats a `Timestamp` by writing
    // the hour, minute, and second to the provided formatter.
    //
    // # Arguments
    //
    // * `f` - A mutable reference to a formatter.
    //
    // Returns `std::fmt::Result` which indicates whether the operation
    // succeeded or failed.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minute ,self.second)
    }
}

#[derive(Debug)]
struct Line {
    timestamp: Timestamp,
    log_level: LogLevel,
    message: String
}

impl Line {
    // Generates a random log level.
    fn gen_log_level(rng: &mut ThreadRng) -> LogLevel {
        match rng.random_range(0..4) {
            0 => LogLevel::Error,
            1 => LogLevel::Warning,
            2 => LogLevel::Info,
            _ => LogLevel::Debug,
        }
    }

    // Generates a random log message.
    fn gen_message(rng: &mut ThreadRng) -> String {
        // Generate the vector of random ASCII characters
        let ascii: Vec<u8> = (0..15)
            .map(|_| rng.random_range(32..=126))
            .collect();
        String::from_utf8_lossy(&ascii).to_string()
    }

    // Generates a random timestamp.
    //
    // The timestamp is in the format of 24-hour clock. The hour is a random
    // number between 0 and 23, the minute is a random number between 0 and 59,
    // and the second is a random number between 0 and 59.
    fn gen_timestamp(rng: &mut ThreadRng) -> Timestamp {
        Timestamp {
            hour: rng.random_range(0..24),
            minute: rng.random_range(0..=59),
            second: rng.random_range(0..=59),
        }
    }

    // Generates a random log line.
    //
    // # Examples
    //23:32:56 Warning ZOcRmBreRrSzUdbC
    //19:54:58 Debug TGhImvBKfStO
    fn gen_line(rng: &mut ThreadRng) -> Line {
        Line {
            timestamp: Line::gen_timestamp(rng),
            log_level: Line::gen_log_level(rng),
            message: Line::gen_message(rng)
        }
    }
}

impl std::fmt::Display for Line {
    // Formats the `Line` struct for display.
    //
    // This implementation of the `fmt` method formats a `Line` by writing
    // the timestamp, log level, and the message to the provided formatter.
    //
    // # Arguments
    //
    // * `f` - A mutable reference to a formatter.
    //
    // Returns `std::fmt::Result` which indicates whether the operation
    // succeeded or failed.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {:?} {}", self.timestamp, self.log_level, self.message)
    }
}

// Generates 1,000,000 random log lines and writes them to a file named
// "example.log".
//
// The generation is done in parallel, and the writing is done in a buffered
// manner. The entire operation is wrapped in a `Result` so that any errors
// that might occur are propagated up the call stack.
pub fn generate() -> std::io::Result<()> {
    // 1. Parallel Line Generation
    // Use Rayon's parallel iterator to generate all lines in parallel.
    // Each thread gets its own random number generator.
    let lines: Vec<String> = (0..1000000)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::rng();
            Line::gen_line(&mut rng).to_string()
        })
        .collect();

    // 2. Efficient File Writing
    // Create the file and wrap it in a BufWriter for buffered I/O.
    let file = File::create("Dummy_file.log")?;
    let mut writer = BufWriter::new(file);

    // Write all the generated lines to the buffered writer.
    for line in lines {
        writeln!(&mut writer, "{}", line)?;
    }

    // The buffer is automatically flushed when `writer` goes out of scope.
    Ok(())
}