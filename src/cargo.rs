use std::error::Error as StdError;
use crate::data::Profile;
use std::path::Path;

pub fn build(path: &Path, profile: Profile) -> Result<(), Error> {
    panic!()
}

#[derive(Debug, Display)]
pub enum Error { }

impl StdError for Error { }
