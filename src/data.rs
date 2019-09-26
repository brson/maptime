use std::error::Error as StdError;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use chrono::{DateTime, Utc};
use crate::commit_list::CommitInput;
use crate::commit_id::CommitId;
use std::time::Duration;

#[derive(Default, Serialize, Deserialize)]
pub struct Data {
    pub unresolved_commits: Vec<CommitInput>,
    pub commits: BTreeMap<CommitId, Commit>,
    pub timings: BTreeMap<CommitId, Vec<Timing>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: CommitId,
    pub date: DateTime<Utc>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timing {
    pub profile: Profile,
    pub start: DateTime<Utc>,
    pub duration: Duration,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Profile {
    FullRelease,
    PartialRelease,
    FullDev,
    PartialDev,
}
