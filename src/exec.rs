use std::error::Error as StdError;
use crate::commit_list::CommitList;
use crate::opts::{Options, Command, GlobalOptions};
use crate::data::Data;

pub fn run_command(opts: &Options) -> Result<(), Error> {
    match opts.cmd {
        Command::ListCommits => {
            list_commits(&opts.global)
        }
        _ => { panic!() }
    }
}

fn list_commits(opts: &GlobalOptions) -> Result<(), Error> {
    let data = Data::load(&opts.db_file)?;

    panic!()
}

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "database error")]
    Data(crate::data::Error),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Data(ref e) => Some(e),
        }
    }
}

impl From<crate::data::Error> for Error {
    fn from(e: crate::data::Error) -> Error {
        Error::Data(e)
    }
}
