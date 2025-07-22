use std::io::Write;
use std::fs::File;
use enum_derived::Rand;
use rand::{Rng};
use random_string;

#[derive(Rand, Debug)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Debug)]
pub struct Timestamp {
    pub hour: u8,
    pub minute: u8,
    pub second: u8
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
    // # Returns
    //
    // Returns `std::fmt::Result` which indicates whether the operation
    // succeeded or failed.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minute ,self.second)
    }
}

#[derive(Debug)]
pub struct Line {
    pub timestamp: Timestamp,
    pub log_level: LogLevel,
    pub message: String
}

impl Line {
    // Generates a random log level.
    fn gen_log_level() -> LogLevel {
        LogLevel::rand()
    }

    // Generates a random log message.
    fn gen_message() -> String {
        random_string::generate_rng(10..20, random_string::charsets::ALPHA)
    }

    // Generates a random timestamp.
    //
    // The timestamp is in the format of 24 hour clock. The hour is a random
    // number between 0 and 23, the minute is a random number between 0 and 59,
    // and the second is a random number between 0 and 59.
    fn gen_timestamp(rng: &mut rand::rngs::ThreadRng) -> Timestamp {
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
    pub fn gen_line(rng: &mut rand::rngs::ThreadRng) -> Line {
        Line {
            timestamp: Line::gen_timestamp(rng),
            log_level: Line::gen_log_level(),
            message: Line::gen_message()
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
    // # Returns
    //
    // Returns `std::fmt::Result` which indicates whether the operation
    // succeeded or failed.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {:?} {}", self.timestamp, self.log_level, self.message)
    }
}

pub fn generate() -> std::io::Result<()> {
    let mut rng = rand::rng();
    File::create("example.log")?;
    let mut f = File::options().append(true).open("example.log")?;
    for _ in 0..1000000 {
        let line = Line::gen_line(& mut rng);
        writeln!(&mut f, "{}", line)?;
    }
    Ok(())
}