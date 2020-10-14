use anyhow::Result;
use clap::Clap;

mod opts;

use crate::opts::{Command, Opts};
use safe_io_lib::{read_file, write_file};

fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();

    match opts.command {
        Command::Write(write_args) => write_file(
            &write_args.path,
            &write_args.content,
            write_args.mode.as_deref(),
        ),
        Command::Read(read_args) => {
            let content = read_file(&read_args.path)?;
            println!("{}", content);
            Ok(())
        },
    }
}
