#![allow(clippy::option_as_ref_deref)]

use anyhow::Result;

use std::io::{stdout, Write};

mod opts;

use crate::opts::{Command, Opts};
use fatrw::{read_file, write_file};

fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse()?;

    match opts.command {
        Command::Write(write_args) => write_file(
            &write_args.path,
            write_args.content.as_bytes(),
            write_args.mode.as_ref().map(String::as_str),
        ),
        Command::Read(read_args) => {
            let content = read_file(&read_args.path)?;
            stdout().write_all(&content)?;
            Ok(())
        }
    }
}
