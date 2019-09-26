use crate::git;
use std::path::Path;
use atomic_blobject::AtomBlob;
use std::error::Error as StdError;
use crate::commit_list::CommitList;
use crate::opts::{Options, Command, GlobalOptions};
use crate::data::Data;
use crate::commit_list::CommitInput;

pub fn run_command(opts: &Options) -> Result<(), Error> {
    match opts.cmd {
        Command::ListCommits => {
            list_commits(&opts.global)
        }
        Command::IngestCommit(ref commit) => {
            ingest_commit(&opts.global, commit.clone())
        }
        Command::IngestCommitList { .. } => {
            panic!()
        }
        Command::ResolveCommits => {
            resolve_commits(&opts.global)
        }
        Command::RunAll => {
            run_all(&opts.global)
        }
    }
}

fn load_data(path: &Path) -> Result<AtomBlob<Data>, Error> {
    Ok(AtomBlob::new(path)?)
}

fn list_commits(opts: &GlobalOptions) -> Result<(), Error> {
    let mut data = load_data(&opts.db_file)?;
    let data = data.get()?;

    let mut some_commits = false;
    if !data.unresolved_commits.is_empty() {
        println!("unresolved commits");
        println!("------------------");
        for commit in &data.unresolved_commits {
            println!("{:?}", commit);
        }
        println!();
        some_commits = true;
    }
    if !data.commits.is_empty() {
        println!("commits");
        println!("-------");
        for commit in &data.commits {
            println!("{:?}", commit);
        }
        println!();
        some_commits = true;
    }

    if !some_commits {
        println!("no commits");
        println!();
    }

    Ok(())
}

fn ingest_commit(opts: &GlobalOptions, commit: CommitInput) -> Result<(), Error> {
    let mut data = load_data(&opts.db_file)?;
    let mut data = data.get_mut()?;

    data.unresolved_commits.push(commit);
    Ok(data.commit()?)
}

fn resolve_commits(opts: &GlobalOptions) -> Result<(), Error> {
    let mut data = load_data(&opts.db_file)?;

    loop {
        let full_commit = {
            let data = data.get()?;

            if let Some(basic_commit) = data.unresolved_commits.last().clone() {
                git::read_commit(&opts.repo_path, &basic_commit)?
            } else {
                break;
            }
        };

        let mut data = data.get_mut()?;
        data.commits.insert(full_commit.id.clone(), full_commit);
        data.unresolved_commits.pop();
        data.commit()?;
    }

    Ok(())
}

fn run_all(opts: &GlobalOptions) -> Result<(), Error> {
    let mut data = load_data(&opts.db_file)?;
    let mut counter = 0;

    {
        let data = data.get()?;
        if !data.unresolved_commits.is_empty() {
            return Err(Error::UnresolvedCommits);
        }
        if data.commits.is_empty() {
            return Err(Error::NoCommits);
        }
    }
    
    loop {
        let mut data = data.get_mut()?;

        data.commit()?;
    }

    println!("done. timed {} commits", counter);

    Ok(())
}

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "loading blobject")]
    AtomBlob(atomic_blobject::Error),
    #[display(fmt = "running git")]
    Git(crate::git::Error),
    #[display(fmt = "database contains unresolved commits. run `maptime resolve-commits`")]
    UnresolvedCommits,
    #[display(fmt = "no commits to test. add with `maptime ingest-commit`")]
    NoCommits,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::AtomBlob(ref e) => Some(e),
            Error::Git(ref e) => Some(e),
            Error::UnresolvedCommits => None,
            Error::NoCommits => None,
        }
    }
}

impl From<atomic_blobject::Error> for Error {
    fn from(e: atomic_blobject::Error) -> Error {
        Error::AtomBlob(e)
    }
}

impl From<crate::git::Error> for Error {
    fn from(e: crate::git::Error) -> Error {
        Error::Git(e)
    }
}
