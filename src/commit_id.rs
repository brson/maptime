use std::convert::AsRef;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use hex::FromHexError;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommitId(String);

impl FromStr for CommitId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let _ = hex::decode(s)?;
        if s.len() != 40 {
            return Err(Error::Length);
        }
        Ok(CommitId(s.to_string()))
    }
}

impl AsRef<str> for CommitId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "git sha hex conversion error")]
    FromHex(FromHexError),
    #[display(fmt = "git sha wrong length")]
    Length,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::FromHex(ref e) => Some(e),
            Error::Length => None,
        }
    }
}

impl From<FromHexError> for Error {
    fn from(v: FromHexError) -> Error {
        Error::FromHex(v)
    }
}
