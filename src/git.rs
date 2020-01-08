use chrono::{DateTime, Utc};
use std::fmt::{self, Display, Formatter};
use std::path::Path;
use std::error::Error as StdError;
use crate::commit_id::CommitId;
use crate::commit_list::CommitInput;
use crate::data::Commit;
use std::process::Command;
use std::str::FromStr;

pub fn read_commit(path: &Path, commit: &CommitInput) -> Result<Commit, Error> {
    let date = read_commit_date(path, commit.id.as_ref())?;

    Ok(Commit {
        id: commit.id.clone(),
        date,
        note: commit.note.clone(),
    })
}

pub fn current_commit(path: &Path) -> Result<CommitId, Error> {
    read_commit_id(path, "HEAD")
}

fn read_commit_date(path: &Path, commit: &str) -> Result<DateTime<Utc>, Error> {
    let stdout = read_commit_stdout(path, commit, "%cD")?;

    let date = DateTime::parse_from_rfc2822(&stdout).map_err(|e| Error::DateParse(e))?;
    let date = DateTime::<Utc>::from(date);

    Ok(date)
}

pub fn read_commit_id(path: &Path, commit: &str) -> Result<CommitId, Error> {
    let stdout = read_commit_stdout(path, commit, "%H")?;

    let id = CommitId::from_str(&stdout).map_err(|e| Error::ReadCommitId(e))?;

    Ok(id)
}

pub fn checkout(path: &Path, commit: &CommitId) -> Result<(), Error> {
    run_git_c(path, "checkout", commit.as_ref(), &[]).map(|_| ())
}

pub fn checkout_file(path: &Path, file: &Path) -> Result<(), Error> {
    let file = file.to_str().ok_or(Error::BadPath)?;
    run_git_c(path, "checkout", "HEAD", &[file, "-f"]).map(|_| ())
}

fn read_commit_stdout(path: &Path, commit: &str, format: &str) -> Result<String, Error> {
    run_git_c(path, "log", commit, &["-1", &format!("--pretty={}", format)])
}

fn run_git_c(path: &Path, gitcmd: &str, commit: &str, args: &[&str]) -> Result<String, Error> {
    let mut cmd = Command::new("git");
    let cmd = cmd
        .arg("-C")
        .arg(path)
        .arg(gitcmd)
        .arg(commit)
        .args(args);

    println!("executing git -C {} {} {} {}",
             path.display(), gitcmd, commit, args.join(" "));

    let out = cmd.output().map_err(|e| Error::GitExec(e))?;

    if !out.status.success() {
        let commit = commit.to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        return Err(Error::GitLog { commit, stderr });
    }

    let stdout = std::str::from_utf8(&out.stdout).map_err(|e| Error::RawDateParse(e))?;
    let stdout = stdout.trim();

    Ok(stdout.to_string())
}

fn run_git(path: &Path, gitcmd: &str, args: &[&str]) -> Result<String, Error> {
    let mut cmd = Command::new("git");
    let cmd = cmd
        .arg("-C")
        .arg(path)
        .arg(gitcmd)
        .args(args);

    println!("executing git -C {} {} {}",
             path.display(), gitcmd, args.join(" "));

    let out = cmd.output().map_err(|e| Error::GitExec(e))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        return Err(Error::Git { stderr });
    }

    let stdout = std::str::from_utf8(&out.stdout).map_err(|e| Error::RawDateParse(e))?;
    let stdout = stdout.trim();

    Ok(stdout.to_string())
}

#[derive(Debug)]
pub enum Error {
    GitExec(std::io::Error),
    GitLog { commit: String, stderr: String },
    Git { stderr: String },
    RawDateParse(std::str::Utf8Error),
    DateParse(chrono::ParseError),
    ReadCommitId(crate::commit_id::Error),
    BadPath,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::GitExec(ref e) => Some(e),
            Error::GitLog { .. } => None,
            Error::Git { .. } => None,
            Error::RawDateParse(ref e) => Some(e),
            Error::DateParse(ref e) => Some(e),
            Error::ReadCommitId(ref e) => Some(e),
            Error::BadPath => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::GitExec(_) => {
                write!(f, "executing git")
            }
            Error::GitLog { ref commit, ref stderr } => {
                write!(f, "failed to read commit {} from git: {}", commit, stderr)
            }
            Error::Git { ref stderr } => {
                write!(f, "failed to run git: {}", stderr)
            }
            Error::RawDateParse(_) => {
                write!(f, "converting git date to UTF-8")
            }
            Error::DateParse(_) => {
                write!(f, "parsing date from git")
            }
            Error::ReadCommitId(_) => {
                write!(f, "reading commit id from git")
            }
            Error::BadPath => {
                write!(f, "bad git checkout file path")
            }
        }
    }
}
