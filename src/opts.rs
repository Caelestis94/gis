use std::path::PathBuf;

use clap::{Parser,command};

#[derive(Debug,Parser)]
#[clap()]
#[command(version, about)]
pub struct Opts {
    /// command to run (identity, workspace, swap, current, check)
    pub command: Vec<String>,
    /// Path to the config file
    #[clap(short = 'c', long = "config")]
    pub config: Option<PathBuf>,
    /// Path to the working directory
    #[clap(short = 'p', long = "pwd")]
    pub pwd: Option<PathBuf>,
}