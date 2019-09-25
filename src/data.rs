use std::error::Error as StdError;
use std::path::Path;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use chrono::{DateTime, Utc};
use crate::commit_list::CommitInput;
use crate::commit_id::CommitId;

pub struct Data {
    unresolved_commits: Vec<CommitInput>,
    commits: BTreeSet<Commit>,
}

#[derive(Eq, PartialEq)]
pub struct Commit {
    id: CommitId,
    time: DateTime<Utc>,
    note: Option<String>,
}

impl Ord for Commit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Commit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl Data {
    pub fn load(path: &Path) -> Result<Data, Error> {
        panic!()
    }
}

#[derive(Debug, Display)]
pub enum Error {
}

impl StdError for Error { }
