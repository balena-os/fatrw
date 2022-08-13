use anyhow::Result;

use clap::Parser;

use std::io::{stdin, stdout, Read, Write};

mod cli;

use fatrw::{copy_file, read_file, write_file};

use crate::cli::{mode_from_string, Cli, Command};

fn main() -> Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .init();

    let args = Cli::parse();

    match args.command {
        Command::Write { path, mode } => {
            let mut input = Vec::new();
            stdin().read_to_end(&mut input)?;
            write_file(&path, &input, mode_from_string(&mode)?)
        }
        Command::Read { path } => {
            let content = read_file(&path)?;
            stdout().write_all(&content)?;
            Ok(())
        }
        Command::Copy { source, dest } => copy_file(&source, &dest),
    }
}
