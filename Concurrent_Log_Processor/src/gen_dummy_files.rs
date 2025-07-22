use enum_derived::Rand;
use rand::Rng;
use random_string;

#[derive(Rand)]
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

impl Line {
    // Generates a random log level.
    fn gen_log_level() -> LogLevel {
        LogLevel::rand()
    }

    // Generates a random log message.
    fn gen_message() -> String {
        random_string::generate_rng(10..20, random_string::charsets::ALPHA)
    }

    fn gen_timestamp() -> Timestamp {
        let mut rng = rand::rng();

        Timestamp {
            hour: rng.random_range(0..24),
            minute: rng.random_range(0..59),
            second: rng.random_range(0..59),
        }
    }

    fn gen_line() -> Line {
        Line {
            timestamp: Line::gen_timestamp(),
            log_level: Line::gen_log_level(),
            message: Line::gen_message()
    }
    }
}


pub fn generate() {
}