use crate::CommitRecord;
use std::path::PathBuf;
use crate::CommitListFile;
use std::env::Args;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::error::Error as StdError;
use crate::commit_list::CommitList;
use std::str::FromStr;
use structopt::StructOpt;
use crate::commit_id::Error as CommitIdError;

/// The command the program will run along with global options
#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(subcommand)]
    cmd: Command,
    db_file: PathBuf,
}

/// The command the program will run
#[derive(Debug, StructOpt)]
pub enum Command {
    IngestListFile {
        file: CommitListFile,
    },
    IngestCommit(CommitRecord),
    CollectGitMeta,
    RunAll,
}
