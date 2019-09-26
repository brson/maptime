use std::error::Error as StdError;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use chrono::{DateTime, Utc};
use crate::commit_list::CommitInput;
use crate::commit_id::CommitId;

#[derive(Default, Serialize, Deserialize)]
pub struct Data {
    pub unresolved_commits: Vec<CommitInput>,
    pub commits: BTreeSet<Commit>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub id: CommitId,
    pub time: DateTime<Utc>,
    pub note: Option<String>,
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
