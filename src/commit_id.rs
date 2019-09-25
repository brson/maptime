use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use hex::FromHexError;

#[derive(Debug)]
pub struct CommitId(String);

impl FromStr for CommitId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let _ = hex::decode(s)?;
        Ok(CommitId(s.to_string()))
    }
}

#[derive(Debug, Display)]
pub struct Error(#[display(fmt = "hex conversion error")] FromHexError);

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        return Some(&self.0);
    }
}

impl From<FromHexError> for Error {
    fn from(v: FromHexError) -> Error {
        Error(v)
    }
}
