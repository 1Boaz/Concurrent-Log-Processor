enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

struct Timestamp {
    hour: u8,
    minute: u8,
    second: u8
}

struct Line {
    timestamp: Timestamp,
    log_level: LogLevel,
    message: String
}

impl Line {}

pub fn generate() {

}