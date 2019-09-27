use std::error::Error as StdError;
use crate::data::Profile;
use std::path::Path;
use crate::data::BuildResult;
use crate::data::Timing;
use crate::data::RebuildType;
use std::process::Command;

pub struct BuildResultPair {
    pub full: Timing,
    pub partial: Option<Timing>,
}

pub fn time_build(path: &Path, profile: Profile) -> Result<BuildResultPair, Error> {
    cargo_clean(path)?;
    let full_result = cargo_time_build(path, profile, RebuildType::Full)?;
    if full_result.result == BuildResult::Failure {
        return Ok(BuildResultPair {
            full: full_result,
            partial: None,
        });
    }
    let partial_result = cargo_time_build(path, profile, RebuildType::Partial)?;
    Ok(BuildResultPair {
        full: full_result,
        partial: Some(partial_result),
    })
}

fn cargo_clean(path: &Path) -> Result<(), Error> {
    panic!()
}

fn cargo_time_build(path: &Path, profile: Profile, rebuild_type: RebuildType) -> Result<Timing, Error> {
    panic!()
}

#[derive(Debug, Display)]
pub enum Error { }

impl StdError for Error { }
