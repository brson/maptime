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
mod exec;
mod opts;

/// A path to a file containing a newline-separated list of commit ID's (SHA1s),
/// pontentially followed by a spacec and arbitrary description
type CommitListFile = PathBuf;

#[derive(Debug, StructOpt)]
struct CommitRecord {
    id: CommitId,
    note: Option<CommitNote>,
}

/// A validated git commit ID (SHA1)
use self::commit_id::CommitId;

/// An abritrary one-line note about a description, from the CommitListFile
type CommitNote = String;

fn main() {
    use opts::Options;

    let opts = Options::from_args();
    println!("{:?}", opts);
}
