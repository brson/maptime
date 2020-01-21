use std::path::{PathBuf, Path};
use std::env::Args;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::error::Error as StdError;
use crate::commit_list::CommitInput;
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
    ResolveCommits,
    RunAll,
    CatchUp,
    FillGaps,
    DumpResults,
    Plot {
        #[structopt(long, default_value = "maptime.svg")]
        file: PathBuf,
        #[structopt(long)]
        no_labels: bool,
    },
    Bisect,
}

#[derive(Debug, StructOpt)]
pub struct GlobalOptions {
    #[structopt(long, default_value = "maptime.json")]
    pub db_file: PathBuf,
    #[structopt(long, default_value = ".")]
    pub repo_path: PathBuf,
    #[structopt(long)]
    pub project_path: Option<PathBuf>,
}

impl GlobalOptions {
    pub fn project_path(&self) -> &Path {
        self.project_path.as_ref().unwrap_or(&self.repo_path)
    }
}
