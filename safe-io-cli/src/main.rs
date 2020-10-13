use anyhow::Result;
use clap::Clap;

mod opts;

use crate::opts::{Command, Opts};
use safe_io_lib::{execute_read, execute_write};

fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();

    match opts.command {
        Command::Write(write_args) => execute_write(
            &write_args.path,
            &write_args.content,
            write_args.mode.as_deref(),
        ),
        Command::Read(read_args) => execute_read(&read_args.path),
    }
}
