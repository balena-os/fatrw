use clap::{App, AppSettings, Arg, SubCommand};

use log::debug;

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

pub struct Opts {
    pub command: Command,
}

pub enum Command {
    Write(WriteArgs),
    Read(ReadArgs),
    Copy(CopyArgs),
}

pub struct WriteArgs {
    pub path: PathBuf,
    pub mode: Option<u32>,
}

pub struct ReadArgs {
    pub path: PathBuf,
}

pub struct CopyArgs {
    pub source: PathBuf,
    pub dest: PathBuf,
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
            .subcommand(
                SubCommand::with_name("copy")
                    .arg(Arg::with_name("source").required(true))
                    .arg(Arg::with_name("dest").required(true)),
            )
            .get_matches();

        let command = match app_matches.subcommand() {
            ("read", Some(matches)) => Command::Read(ReadArgs {
                path: path_buf(matches.value_of("path").unwrap()),
            }),
            ("write", Some(matches)) => Command::Write(WriteArgs {
                path: path_buf(matches.value_of("path").unwrap()),
                mode: mode_from_string(matches.value_of("mode"))?,
            }),
            ("copy", Some(matches)) => Command::Copy(CopyArgs {
                source: path_buf(matches.value_of("source").unwrap()),
                dest: path_buf(matches.value_of("dest").unwrap()),
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

pub fn mode_from_string(mode: Option<&str>) -> Result<Option<u32>> {
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
