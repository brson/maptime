use crate::data::BuildResult;
use std::str::FromStr;
use crate::cargo;
use std::cmp::{self, Ordering};
use crate::opts::GlobalOptions;
use std::error::Error as StdError;
use crate::gnuplot::{PlotData, Entry};
use crate::data::{Profile, RebuildType};
use std::mem;
use std::time::Duration;
use crate::git;
use std::path::Path;
use std::num;
use crate::commit_id::CommitId;

#[derive(Debug, Clone)]
struct BisectRange {
    profile: Profile,
    rebuild_type: RebuildType,
    first: Entry,
    last: Entry,
    diff: Duration,
}

pub fn bisect(opts: &GlobalOptions, data: PlotData) -> Result<(), Error> {
    let range = find_biggest_range(opts, data)?;
    println!("bisecting {:#?}", range);
    bisect_range(opts, range)
}

fn bisect_range(opts: &GlobalOptions, range: BisectRange) -> Result<(), Error> {
    let hysteresis = range.diff / 10;
    let max = cmp::max(range.first.duration, range.last.duration);
    let min = cmp::min(range.first.duration, range.last.duration);
    let mid = max - (range.diff / 2);
    let ord = range.first.duration.cmp(&range.last.duration);
    let is_new = |d: Duration| {
        if ord == Ordering::Less {
            d > mid + hysteresis
        } else {
            d < mid - hysteresis
        }
    };

    println!("max: {:?}, min: {:?}, mid: {:?}, ord: {:?}, hyst: {:?}",
             max, min, mid, ord, hysteresis);

    let out = git::run_git(&opts.repo_path, "bisect",
                           &["start",
                             range.last.commit.id.as_str(),
                             range.first.commit.id.as_str(),
                             "--term-old=old",
                             "--term-new=new"])?;
    println!("{}", out);

    if !still_bisecting(&out) {
        git::run_git(&opts.repo_path, "bisect", &["reset"])?;
        return Ok(());
    }

    let mut commit = parse_commit_from_stdout(&out)?;

    let profile = range.profile;
    let project_path = opts.project_path();

    loop {
        let results = cargo::time_build(project_path, profile)?;

        if let Some(touched) = results.touched {
            git::checkout_file(project_path, &touched)?;
        }

        if results.full.result == BuildResult::Failure {
            println!("bad build - bisect skip");
            git::run_git(&opts.repo_path, "bisect", &["skip"]);
            continue;
        }

        let timing;
        if range.rebuild_type == RebuildType::Full {
            timing = results.full;
        } else {
            timing = results.partial.ok_or_else(|| Error::NoPartialBuild(commit.clone()))?;
        }

        let out;
        if is_new(timing.duration) {
            out = git::run_git(&opts.repo_path, "bisect", &["new"])?;
        } else {
            out = git::run_git(&opts.repo_path, "bisect", &["old"])?;
        }

        println!("{}", out);
        println!("duration: {:?}", timing.duration);

        if !still_bisecting(&out) {
            git::run_git(&opts.repo_path, "bisect", &["reset"])?;
            return Ok(());
        }

        commit = parse_commit_from_stdout(&out)?;
    }

    Ok(())
}

fn still_bisecting(s: &str) -> bool {
    if s.contains("Bisecting:") {
        true
    } else {
        false
    }
}

fn parse_commit_from_stdout(s: &str) -> Result<CommitId, Error> {
    for line in s.lines() {
        if line.starts_with("[") && line.len() > 40 {
            let s = &line[1..41];
            assert!(s.len() == 40);
            let c = CommitId::from_str(s).map_err(Error::CommitIdParse)?;
            return Ok(c);
        }
    }
    return Err(Error::BisectParse);
}

fn find_biggest_range(opts: &GlobalOptions, data: PlotData) -> Result<BisectRange, Error> {
    let mut biggest: Option<BisectRange> = None;
    for series in data.0 {
        let mut prev: Option<Entry> = None;
        for entry in series.values {
            if let Some(p) = prev {
                if is_parent(opts, &p.commit.id, &entry.commit.id)? {
                    println!("{} is parent of {}", p.commit.id.as_str(), entry.commit.id.as_str());
                    // No bisection to be done
                    prev = Some(entry);
                    continue;
                }

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

fn is_parent(opts: &GlobalOptions, maybe_parent: &CommitId, next: &CommitId) -> Result<bool, Error> {
    let actual_parent = git::get_parent(opts.project_path(), next)?;
    Ok(*maybe_parent == actual_parent)
}

#[derive(Display, Debug)]
pub enum Error {
    #[display(fmt = "not enough commits to bisect")]
    NotEnoughCommits,
    #[display(fmt = "git error")]
    Git(crate::git::Error),
    #[display(fmt = "no partial build for {} during bisect", "_0.as_str()")]
    NoPartialBuild(CommitId),
    #[display(fmt = "parsing commit")]
    CommitIdParse(crate::commit_id::Error),
    #[display(fmt = "parsing bisect output")]
    BisectParse,
    #[display(fmt = "running cargo")]
    Cargo(crate::cargo::Error),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::NotEnoughCommits => None,
            Error::Git(ref e) => Some(e),
            Error::NoPartialBuild(_) => None,
            Error::CommitIdParse(ref e) => Some(e),
            Error::BisectParse => None,
            Error::Cargo(ref e) => Some(e),
        }
    }
}

impl From<crate::git::Error> for Error {
    fn from(e: crate::git::Error) -> Error {
        Error::Git(e)
    }
}

impl From<crate::cargo::Error> for Error {
    fn from(e: crate::cargo::Error) -> Error {
        Error::Cargo(e)
    }
}
