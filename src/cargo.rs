use std::error::Error as StdError;
use crate::data::Profile;
use std::path::Path;
use crate::data::BuildResult;

pub fn build(path: &Path, profile: Profile) -> Result<BuildResult, Error> {
    panic!()
}

#[derive(Debug, Display)]
pub enum Error { }

impl StdError for Error { }
