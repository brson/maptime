#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate structopt;

use std::env::Args;
use std::path::PathBuf;
use std::collections::BTreeMap;
use structopt::StructOpt;

mod commit_id;
mod commit_list;
mod cli;

/// The command the program will run along with global options
#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(subcommand)]
    cmd: Command,
    db_file: PathBuf,
}

/// The command the program will run
#[derive(Debug, StructOpt)]
enum Command {
    IngestListFile {
        file: CommitListFile,
    },
    IngestCommit {
        id: CommitId,
        note: Option<CommitNote>
    },
    CollectGitMeta,
    RunAll,
}

/// A path to a file containing a newline-separated list of commit ID's (SHA1s),
/// pontentially followed by a spacec and arbitrary description
type CommitListFile = PathBuf;

struct CommitRecord {
    id: CommitId,
    note: Option<CommitNote>,
}

/// A validated git commit ID (SHA1)
use self::commit_id::CommitId;

/// An abritrary one-line note about a description, from the CommitListFile
type CommitNote = String;

fn main() {
    let opts = Options::from_args();
    println!("{:?}", opts);
}


/// Command execution
mod exec {
    use super::{Options, Command};

    use super::commit_list::CommitList;
    
    fn run_command(opts: &Options) { }

}
