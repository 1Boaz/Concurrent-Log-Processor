//! Command-line argument parsing and configuration for the Concurrent Log Processor.
//!
//! This module defines the command-line interface using the `clap` derive macros.
//! It handles argument parsing, validation, and provides help/usage information.

use clap_derive::{Parser, Subcommand, Args};

/// Represents the main subcommands available in the application.
///
/// This enum defines the different operations that can be performed by the application.
/// Each variant corresponds to a different subcommand with its own set of arguments.
#[derive(Subcommand, Debug)]
pub enum EntityType {
    /// Generate a test log file with random log entries.
    ///
    /// This command creates a new log file with the specified number of random log entries,
    /// which can be used for testing and demonstration purposes.
    Generate(GenerateCommand),

    /// Process a log file with configurable filtering and concurrency.
    ///
    /// Processes log files with the format:
    /// `YYYY-MM-DDTHH:MM:SS LOG_LEVEL(One of INFO, WARNING, ERROR, or DEBUG) MESSAGE`
    ///
    /// The processing can be customized with thread count and log level filtering.
    Process(ProcessCommand)
}

/// Arguments for the `generate` subcommand.
///
/// This struct defines the command-line arguments specific to generating test log files.
#[derive(Args, Debug)]
pub struct GenerateCommand {
    /// Name for the file to be generated.
    /// If not provided, a default name with timestamp will be used.
    #[clap(short, long)]
    pub filename: Option<String>,

    /// Number of log entries to generate in the file.
    /// If not provided, a default number of lines will be generated.
    #[clap(short, long)]
    pub lines: Option<u32>,
}

/// Arguments for the `process` subcommand.
///
/// This struct defines the command-line arguments for processing log files.
#[derive(Args, Debug)]
pub struct ProcessCommand {
    /// Path to the log file that needs to be processed.
    /// This is a required argument.
    #[clap(short, long)]
    pub file: String,

    /// Number of threads to use for processing the log file.
    /// If not specified, a default number of threads will be used.
    #[clap(short, long)]
    pub threads: Option<u128>,

    /// Log level to filter by (e.g., "INFO", "ERROR", "WARNING", "DEBUG").
    /// If not specified, all log levels will be processed.
    #[clap(short, long)]
    pub log_level: Option<String>,
}

/// Main command-line interface structure.
///
/// This is the root command that contains all possible subcommands.
/// It uses clap's derive macros to automatically generate the command-line interface.
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Concurrent Log Processor - A tool for generating and processing log files with configurable concurrency",
    long_about = "A command-line utility that provides functionality to generate test log files and
process them with configurable concurrency settings. It supports filtering log entries by log level
and can process large log files efficiently using multiple threads."
)]
pub struct Concurrent_Log_Processor {
    /// The subcommand to execute.
    #[clap(subcommand)]
    pub entity_type: EntityType,
}
