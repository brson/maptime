use crate::data::Commit;
use std::time::Duration;
use chrono::{DateTime, Utc};
use std::error::Error as StdError;
use std::path::Path;
use crate::data::{Profile, RebuildType};

pub struct PlotData(pub Vec<Series>);

pub struct Series {
    pub profile: Profile,
    pub rebuild_type: RebuildType,
    pub values: Vec<Entry>,
}

pub struct Entry {
    pub commit: Commit,
    pub duration: Duration,
}

pub fn plot(data: PlotData, file: &Path) -> Result<(), Error> {
    panic!()
}

#[derive(Debug, Display)]
pub enum Error {
}

impl StdError for Error { }
