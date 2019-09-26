use std::path::Path;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use crate::commit_id::CommitId;

#[derive(Debug, StructOpt, Serialize, Deserialize)]
pub struct CommitInput {
    id: CommitId,
    note: Option<String>,
}

/// List of commits as described by the user,
/// Binary-sorted and deduplicated
pub struct CommitList(Vec<CommitInput>);

impl CommitList {
    pub fn ingest(file: &Path) -> Result<Self, Error> {
        panic!()
    }
}

#[derive(Debug)]
pub struct Error;

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        panic!()
    }
}
