use crate::opts::GlobalOpts;
use std::error::Error as StdError;
use crate::gnuplot::{PlotData, Entry};
use crate::data::{Profile, RebuildType};
use std::mem;
use std::time::Duration;
use crate::git;
use std::path::Path;

#[derive(Debug, Clone)]
struct BisectRange {
    profile: Profile,
    rebuild_type: RebuildType,
    first: Entry,
    last: Entry,
    diff: Duration,
}

pub fn bisect(opts: &GlobalOpts, data: PlotData) -> Result<(), Error> {
    let range = find_biggest_range(data)?;
    println!("bisecting {:#?}", range);
    bisect_range(opts, range)
}

fn bisect_range(opts: &GlobalOpts, range: BisectRange) -> Result<(), Error> {
    git::run_git(opts.repo_path, "bisect",
                 &["start",
                   range.last.commit.id.as_str(),
                   range.first.commit.id.as_str()])?;
    panic!()
}

fn find_biggest_range(data: PlotData) -> Result<BisectRange, Error> {
    let mut biggest: Option<BisectRange> = None;
    for series in data.0 {
        let mut prev: Option<Entry> = None;
        for entry in series.values {
            if let Some(p) = prev {
                let mut t1 = p.duration;
                let mut t2 = entry.duration;
                if t1 > t2 {
                    mem::swap(&mut t1, &mut t2);
                }
                let diff = t2 - t1;
                if let Some(cur_big) = biggest.clone() {
                    if cur_big.diff < diff {
                        biggest = Some(BisectRange {
                            profile: series.profile,
                            rebuild_type: series.rebuild_type,
                            first: p,
                            last: entry.clone(),
                            diff: diff,
                        });
                    }
                } else {
                    biggest = Some(BisectRange {
                        profile: series.profile,
                        rebuild_type: series.rebuild_type,
                        first: p,
                        last: entry.clone(),
                        diff: diff,
                    });
                }
                prev = Some(entry);
            } else {
                prev = Some(entry);
            }
        }
    }
    biggest.ok_or(Error::NotEnoughCommits)
}

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "not enough commits to bisect")]
    NotEnoughCommits,
}

impl StdError for Error { }
