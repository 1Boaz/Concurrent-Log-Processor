#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Debug)]
pub struct Timestamp {
    pub year: u16,
    pub month: u8,
    pub day: u8,
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
    // Returns `std::fmt::Result` which indicates whether the operation
    // succeeded or failed.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{}-{}T{}:{}:{}",self.year,self.month,self.day, self.hour, self.minute ,self.second)
    }
}

#[derive(Debug)]
pub struct Line {
    pub timestamp: Timestamp,
    pub log_level: LogLevel,
    pub message: String
}