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
    ListCommits,
    IngestCommit(CommitInput),
    IngestCommitList {
        file: PathBuf,
    },
    CollectGitMeta,
    RunAll,
}

#[derive(Debug, StructOpt)]
pub struct GlobalOptions {
    #[structopt(long, default_value = "maptime.json")]
    pub db_file: PathBuf,
    #[structopt(long, default_value = ".")]
    pub repo_path: PathBuf,
}
