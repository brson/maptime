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
    pub timings: BTreeMap<CommitId, Vec<TimingSet>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: CommitId,
    pub date: DateTime<Utc>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimingSet {
    pub full_release: Timing,
    pub partial_release: Timing,
    pub full_dev: Timing,
    pub partial_dev: Timing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timing {
    pub start: DateTime<Utc>,
    pub duration: Duration,
}
