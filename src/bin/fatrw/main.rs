use anyhow::Result;

use std::io::{stdin, stdout, Read, Write};

mod opts;

use crate::opts::{Command, Opts};
use fatrw::{copy_file, read_file, write_file};

fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse()?;

    match opts.command {
        Command::Write(write_args) => {
            let mut input = Vec::new();
            stdin().read_to_end(&mut input)?;
            write_file(&write_args.path, &input, write_args.mode)
        }
        Command::Read(read_args) => {
            let content = read_file(&read_args.path)?;
            stdout().write_all(&content)?;
            Ok(())
        }
        Command::Copy(copy_args) => copy_file(&copy_args.source, &copy_args.dest),
    }
}
