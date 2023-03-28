use anyhow::Result;

use clap::Parser;

use std::io::{stdin, stdout, Read, Write};

mod cli;

use fatrw::copy::copy_file;
use fatrw::read::read_file;
use fatrw::write::write_file;

use crate::cli::{mode_from_string, Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logging(&cli);

    match cli.command {
        Command::Write { path, mode } => {
            let mut input = Vec::new();
            stdin().read_to_end(&mut input)?;
            write_file(path, &input, mode_from_string(&mode)?)
        }
        Command::Read { path } => {
            let content = read_file(path)?;
            stdout().write_all(&content)?;
            Ok(())
        }
        Command::Copy { source, dest } => copy_file(source, dest),
    }
}

fn init_logging(cli: &Cli) {
    let mut builder = env_logger::builder();

    builder.format_timestamp(None);
    builder.format_target(false);

    if cli.debug {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }

    builder.init();
}
