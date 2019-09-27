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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Commit {
    pub id: CommitId,
    pub date: DateTime<Utc>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timing {
    pub profile: Profile,
    pub rebuild_type: RebuildType,
    pub start: DateTime<Utc>,
    pub duration: Duration,
    pub result: BuildResult,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum BuildResult { Success, Failure }

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum Profile { Dev, Release }

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum RebuildType { Full, Partial }

impl AsRef<str> for Profile {
    fn as_ref(&self) -> &str {
        match self {
            Profile::Dev => "dev",
            Profile::Release => "release",
        }
    }
}

impl AsRef<str> for RebuildType {
    fn as_ref(&self) -> &str {
        match self {
            RebuildType::Full => "full",
            RebuildType::Partial => "partial",
        }
    }
}
