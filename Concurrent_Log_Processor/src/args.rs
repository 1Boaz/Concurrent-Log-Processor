use clap_derive::{Parser, Subcommand, Args};

#[derive(Subcommand, Debug)]
pub enum EntityType {
    /// Generate a test file
    Generate(GenerateCommand),
    /// Processes log file with by this format: YYYY-MM-DDTHH:MM:SS LOG_LEVEL(One of INFO, WARNING, ERROR, or DEBUG) MESSAGE(string)
    Process(ProcessCommand)
}

#[derive(Args, Debug)]
pub struct GenerateCommand {
    /// Name for the file to be generated
    pub filename: Option<String>,
    /// Number of lines the file will be generated with
    pub lines: Option<u32>,
}

#[derive(Args, Debug)]
pub struct ProcessCommand {
    /// The name of the file to process
    pub file: String,
    /// Number of threads the program will use to process the file
    pub threads: Option<u128>,
    /// Log level to filter by
    pub log_level: Option<String>,
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Concurrent_Log_Processor {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}
