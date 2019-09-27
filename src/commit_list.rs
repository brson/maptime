use std::str::FromStr;
use std::path::Path;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use crate::commit_id::CommitId;

#[derive(Debug, StructOpt, Serialize, Deserialize, Clone)]
pub struct CommitInput {
    pub id: CommitId,
    pub note: Option<String>,
}

impl FromStr for CommitInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<CommitInput, Error> {
        if s.len() >= 40 {
            let commit_str = &s[..40];
            let commit = CommitId::from_str(commit_str)?;
            let note_str = &s[40..].trim();
            let note_str = if note_str.len() > 0 {
                Some(note_str.to_string())
            } else {
                None
            };
            Ok(CommitInput {
                id: commit,
                note: note_str,
            })
        } else {
            Err(Error::Length)
        }
    }
}

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "commit input too short")]
    Length,
    #[display(fmt = "parsing commit ID")]
    CommitId(crate::commit_id::Error),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Length => None,
            Error::CommitId(ref e) => Some(e),
        }
    }
}

impl From<crate::commit_id::Error> for Error {
    fn from(e: crate::commit_id::Error) -> Error {
        Error::CommitId(e)
    }
}
