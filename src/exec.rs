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
        Command::CollectGitMeta => {
            collect_git_meta(&opts.global)
        }
        _ => { panic!() }
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

fn collect_git_meta(opts: &GlobalOptions) -> Result<(), Error> {
    use crate::git;

    let mut data = load_data(&opts.db_file)?;
    loop {
        let mut data = data.get_mut()?;

        if let Some(basic_commit) = data.unresolved_commits.last().clone() {
            let full_commit = git::read_commit(&opts.repo_path, &basic_commit)?;

            data.commits.insert(full_commit);
            
            data.unresolved_commits.pop();
        } else {
            break;
        }

        data.commit()?;
    }

    Ok(())
}

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "loading blobject")]
    AtomBlob(atomic_blobject::Error),
    #[display(fmt = "running git")]
    Git(crate::git::Error),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::AtomBlob(ref e) => Some(e),
            Error::Git(ref e) => Some(e),
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
