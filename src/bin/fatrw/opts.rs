use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
#[clap()]
pub enum Command {
    #[clap(alias = "w")]
    Write(WriteArgs),
    #[clap(alias = "r")]
    Read(ReadArgs),
}

#[derive(Parser, Debug)]
#[clap()]
pub struct WriteArgs {
    pub path: PathBuf,
    pub content: String,
    pub mode: Option<String>,
}

#[derive(Parser, Debug)]
#[clap()]
pub struct ReadArgs {
    pub path: PathBuf,
}
