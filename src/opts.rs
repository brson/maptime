use std::path::PathBuf;
use std::env::Args;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::error::Error as StdError;
use crate::commit_list::{CommitInput, CommitList};
use std::str::FromStr;
use structopt::StructOpt;

/// The command the program will run along with global options
#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(subcommand)]
    pub cmd: Command,
    #[structopt(flatten)]
    pub global: GlobalOptions,
}

/// The command the program will run
#[derive(Debug, StructOpt)]
pub enum Command {
    IngestListFile {
        file: PathBuf,
    },
    IngestCommit(CommitInput),
    ListCommits,
    CollectGitMeta,
    RunAll,
}

#[derive(Debug, StructOpt)]
pub struct GlobalOptions {
    #[structopt(default_value = "maptime.json")]
    pub db_file: PathBuf,
}
