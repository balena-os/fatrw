use clap::{App, AppSettings, Arg, SubCommand};

use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

pub struct Opts {
    pub command: Command,
}

pub enum Command {
    Write(WriteArgs),
    Read(ReadArgs),
}

pub struct WriteArgs {
    pub path: PathBuf,
    pub mode: Option<String>,
}

pub struct ReadArgs {
    pub path: PathBuf,
}

impl Opts {
    pub fn parse() -> Result<Self> {
        let app_matches = App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(SubCommand::with_name("read").arg(Arg::with_name("path").required(true)))
            .subcommand(
                SubCommand::with_name("write")
                    .arg(Arg::with_name("mode").short("m").long("mode"))
                    .arg(Arg::with_name("path").required(true)),
            )
            .get_matches();

        let command = match app_matches.subcommand() {
            ("read", Some(matches)) => Command::Read(ReadArgs {
                path: path_buf(matches.value_of("path").unwrap()),
            }),
            ("write", Some(matches)) => Command::Write(WriteArgs {
                path: path_buf(matches.value_of("path").unwrap()),
                mode: matches.value_of("mode").map(str::to_string),
            }),
            _ => {
                bail!("No subcommand")
            }
        };

        Ok(Opts { command })
    }
}

fn path_buf(path: &str) -> PathBuf {
    Path::new(path).to_path_buf()
}
