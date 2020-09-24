use anyhow::Result;
use clap::Clap;

mod checksum;
mod fs;
mod opts;
mod random;
mod read;
mod write;

use crate::opts::{Command, Opts};
use crate::read::execute_read;
use crate::write::execute_write;

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.command {
        Command::Write(write_args) => execute_write(write_args),
        Command::Read(read_args) => execute_read(read_args),
    }
}
