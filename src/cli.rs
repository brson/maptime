use super::{Options, Command, CommitNote, CommitId};
use std::path::PathBuf;
use super::CommitListFile;
use std::env::Args;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::error::Error as StdError;
use super::commit_list::CommitList;
use std::str::FromStr;
use structopt::StructOpt;
use super::commit_id::Error as CommitIdError;

type RawCommitId = String;

#[derive(StructOpt)]
struct ClOptions {
    #[structopt(subcommand)]
    cmd: ClCommand,
    db_file: PathBuf,
}

#[derive(StructOpt)]
enum ClCommand {
    IngestListFile {
        file: CommitListFile,
    },
    IngestCommit {
        id: RawCommitId,
        note: Option<CommitNote>
    },
    CollectGitMeta,
    RunAll,
}

pub fn parse_command_line_options() -> Result<Options, Error> {
    let clopts = ClOptions::from_args();

    Ok(Options::try_from(clopts)?)
}

impl TryFrom<ClOptions> for Options {
    type Error = Error;

    fn try_from(clopts: ClOptions) -> Result<Options, Self::Error> {
        Ok(Options {
            cmd: match clopts.cmd {
                ClCommand::IngestListFile { file } => Command::IngestListFile { file },
                ClCommand::IngestCommit { id, note } => {
                    Command::IngestCommit { id: CommitId::from_str(&id)?, note }
                },
                ClCommand::CollectGitMeta => Command::CollectGitMeta,
                ClCommand::RunAll => Command::RunAll,
                
            },
            db_file: clopts.db_file,
        })
    }
}

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "invalid commit id")]
    CommitIdError(CommitIdError),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::CommitIdError(e) => Some(e),
        }
    }
}

impl From<CommitIdError> for Error {
    fn from(e: CommitIdError) -> Error {
        Error::CommitIdError(e)
    }
}
