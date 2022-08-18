use clap::{Parser, Subcommand};

use log::debug;

use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "fatrw")]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

    /// Print debug information
    #[clap(short, long)]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Read a file
    Read {
        #[clap(value_parser)]
        path: PathBuf,
    },
    /// Write a file reading from a Unix pipe
    #[clap(arg_required_else_help = true)]
    Write {
        #[clap(value_parser)]
        path: PathBuf,

        #[clap(short, long, value_parser)]
        mode: Option<String>,
    },
    /// Copy a file
    Copy {
        #[clap(value_parser)]
        source: PathBuf,

        #[clap(value_parser)]
        dest: PathBuf,
    },
}

pub fn mode_from_string(mode: &Option<String>) -> Result<Option<u32>> {
    Ok(if let Some(octal_str) = mode {
        let octal_mode = parse_file_mode(octal_str)?;
        debug!("File mode: {:o}", octal_mode);
        Some(octal_mode)
    } else {
        None
    })
}

fn parse_file_mode(octal_str: &str) -> Result<u32> {
    u32::from_str_radix(octal_str, 8).context("Parsing file mode failed")
}
