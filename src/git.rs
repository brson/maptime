use chrono::{DateTime, Utc};
use std::fmt::{self, Display, Formatter};
use std::path::Path;
use std::error::Error as StdError;
use crate::commit_id::CommitId;
use crate::commit_list::CommitInput;
use crate::data::Commit;

pub fn read_commit(path: &Path, commit: &CommitInput) -> Result<Commit, Error> {
    use std::process::Command;
    let mut cmd = Command::new("git");
    let cmd = cmd
        .arg("-C")
        .arg(path)
        .arg("log")
        .arg(commit.id.as_ref())
        .arg("--pretty=%cD")
        .arg("-1");

    println!("executing git -C {} log {} --pretty=%cD -1 ", path.display(), commit.id.as_ref());

    let out = cmd.output().map_err(|e| Error::GitExec(e))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        return Err(Error::GitLog(commit.id.clone(), stderr));
    }

    let date = std::str::from_utf8(&out.stdout).map_err(|e| Error::RawDateParse(e))?;
    let date = date.trim();
    let date = DateTime::parse_from_rfc2822(date).map_err(|e| Error::DateParse(e))?;
    let date = DateTime::<Utc>::from(date);

    Ok(Commit {
        id: commit.id.clone(),
        date,
        note: commit.note.clone(),
    })
}

pub fn current_commit(path: &Path) -> Result<CommitId, Error> {
    panic!()
}

pub fn checkout(path: &Path, commit: &CommitId) -> Result<(), Error> {
    panic!()
}

#[derive(Debug)]
pub enum Error {
    GitExec(std::io::Error),
    GitLog(CommitId, String),
    RawDateParse(std::str::Utf8Error),
    DateParse(chrono::ParseError),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::GitExec(ref e) => Some(e),
            Error::GitLog(..) => None,
            Error::RawDateParse(ref e) => Some(e),
            Error::DateParse(ref e) => Some(e),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::GitExec(_) => {
                write!(f, "executing git")
            }
            Error::GitLog(ref commit, ref stderr) => {
                write!(f, "failed to read commit {} from git: {}", commit.as_ref(), stderr)
            }
            Error::RawDateParse(_) => {
                write!(f, "converting git date to UTF-8")
            }
            Error::DateParse(_) => {
                write!(f, "parsing date from git")
            }
        }
    }
}
