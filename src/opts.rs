use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap()]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap, Debug)]
#[clap()]
pub enum Command {
    #[clap(alias = "w")]
    Write(WriteArgs),
    #[clap(alias = "r")]
    Read(ReadArgs),
}

#[derive(Clap, Debug)]
#[clap()]
pub struct WriteArgs {
    pub path: PathBuf,
    pub content: String,
    pub mode: Option<String>,
}

#[derive(Clap, Debug)]
#[clap()]
pub struct ReadArgs {
    pub path: PathBuf,
}
