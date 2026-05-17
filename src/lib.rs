//! # devrunner-cli
//!
//! Universal task runner for modern development.
//!
//! Automatically detects the project's package manager or build tool
//! and runs commands through the appropriate tool.

pub mod cli;
pub mod config;
pub mod detectors;
pub mod error;
pub mod http;
pub mod output;
pub mod runner;
pub mod update;

pub use cli::Cli;
pub use config::Config;
pub use detectors::DetectedRunner;
pub use error::RunError;
