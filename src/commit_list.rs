use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use super::CommitListFile;
use super::CommitNote;

/// List of commits as described by the user,
/// Binary-sorted and deduplicated
pub struct CommitList(Vec<CommitNote>);

impl CommitList {
    pub fn ingest(file: &CommitListFile) -> Result<Self, Error> {
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

