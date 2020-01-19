use crate::commit_id::CommitId;
use std::convert::TryFrom;
use crate::gnuplot::{self, PlotData, Series, Entry};
use crate::data::{BuildResult, Profile, RebuildType, Timing};
use crate::cargo;
use std::time::{Instant, Duration};
use chrono::{DateTime, Utc};
use crate::git;
use std::path::Path;
use atomic_blobject::AtomBlob;
use std::error::Error as StdError;
use crate::opts::{Options, Command, GlobalOptions};
use crate::data::Data;
use crate::commit_list::CommitInput;
use crate::bisect;

pub fn run_command(opts: &Options) -> Result<(), Error> {
    match opts.cmd {
        Command::ListCommits => {
            list_commits(&opts.global)
        }
        Command::IngestCommit(ref commit) => {
            ingest_commit(&opts.global, commit.clone())
        }
        Command::IngestCommitList { ref file } => {
            ingest_commit_list(&opts.global, file)
        }
        Command::ResolveCommits => {
            resolve_commits(&opts.global)
        }
        Command::RunAll => {
            run_all(&opts.global)
        }
        Command::CatchUp => {
            catch_up(&opts.global)
        }
        Command::FillGaps => {
            fill_gaps(&opts.global)
        }
        Command::DumpResults => {
            dump_results(&opts.global)
        }
        Command::Plot { ref file } => {
            plot(&opts.global, file)
        }
        Command::Bisect => {
            bisect(&opts.global)
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
        let commits = data.sorted_commits();
        
        println!("commits");
        println!("-------");
        for commit in &commits {
            let commit = data.commits.get(commit);
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

fn ingest_commit_list(opts: &GlobalOptions, file: &Path) -> Result<(), Error> {
    let list = parse_list::from_file_lines::<CommitInput>(file).map_err(|e| Error::CommitListIo(e))?;
    let list: Vec<Result<CommitInput, _>> = list.collect();
    let list: Result<Vec<CommitInput>, _> = list.into_iter().collect();
    let list = list.map_err(|e| Error::CommitParse(e))?;

    let mut data = load_data(&opts.db_file)?;
    let mut data = data.get_mut()?;
    data.unresolved_commits.extend(list);

    Ok(())
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
    let data = data.get()?;

    let mut plan = vec![];
    for commit in data.commits.keys() {
        plan.push((commit.clone(), 1));
    }

    drop(data);

    run(opts, &RunPlan(plan))
}

fn catch_up(opts: &GlobalOptions) -> Result<(), Error> {
    let mut data = load_data(&opts.db_file)?;
    let data = data.get()?;

    let commits = data.sorted_commits();

    let mut max = 0;
    for commit in &commits {
        let count = data.timings.get(commit).unwrap_or(&vec![]).len();
        max = count.max(max);
    }

    let mut plan = vec![];
    for commit in &commits {
        let count = data.timings.get(commit).unwrap_or(&vec![]).len();
        if count < max {
            plan.push((commit.clone(), u32::try_from(max - count).expect("small timing count")));
        }
    }

    println!("{:#?}", plan);

    plan.sort_by_key(|p| p.1);

    let mut new_plan = vec![];
    loop {
        let mut keep_going = false;
        for &mut (ref commit, ref mut count) in &mut plan {
            if *count > 0 {
                *count -= 1;
                new_plan.push((commit.clone(), 1));
                keep_going = true;
            }
        }
        if !keep_going { break }
    }

    drop(data);

    println!("catch-up plan: {:#?}", new_plan);

    run(opts, &RunPlan(new_plan))
}

fn fill_gaps(opts: &GlobalOptions) -> Result<(), Error> {
    let mut data = load_data(&opts.db_file)?;
    let data = data.get()?;

    let commits = data.sorted_commits();

    let mut plan = vec![];
    for commit in &commits {
        if !data.timings.get(commit).is_some() {
            plan.push((commit.clone(), 1));
        }
    }

    drop(data);

    run(opts, &RunPlan(plan))
}

struct RunPlan(Vec<(CommitId, u32)>);

fn run(opts: &GlobalOptions, plan: &RunPlan) -> Result<(), Error> {
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

    let start_commit = git::current_commit(&opts.repo_path)?;
    println!("saving start commit {}", start_commit.as_ref());

    for &(ref commit, count) in &plan.0 {
        println!("checking out {}", commit.as_ref());
        git::checkout(&opts.repo_path, &commit)?;

        let profiles = [Profile::Dev, Profile::Release];

        for _ in 0..count {
            for profile in profiles.iter().cloned() {
                let project_path = opts.project_path.as_ref().unwrap_or(&opts.repo_path);
                let results = cargo::time_build(project_path, profile)?;

                let mut data = data.get_mut()?;
                data.timings.entry(commit.clone()).or_insert(vec![]).push(results.full);

                if let Some(partial_timing) = results.partial {
                    data.timings.entry(commit.clone()).or_insert(vec![]).push(partial_timing);
                }

                data.commit()?;

                if let Some(touched) = results.touched {
                    // NB project_path, not repo_path
                    git::checkout_file(project_path, &touched)?;
                }
            }
        }

        counter += 1;
    }

    println!("restoring start commit {}", start_commit.as_ref());
    git::checkout(&opts.repo_path, &start_commit)?;

    println!("done. timed {} commits", counter);

    Ok(())
}

fn dump_results(opts: &GlobalOptions) -> Result<(), Error> {
    let mut data = load_data(&opts.db_file)?;
    let data = data.get()?;

    let commits = data.sorted_commits();

    for commit in commits {
        println!("commit {:?}", commit);
        let timings = data.timings.get(&commit);
        if let Some(timings) = timings {
            for timing in timings {
                println!("  {:?}", timing);
            }
        } else {
            println!("  no timings");
        }
    }

    Ok(())
}

fn plot(opts: &GlobalOptions, plotfile: &Path) -> Result<(), Error> {
    let plotdata = get_plot_data(opts)?;
    Ok(gnuplot::plot(plotdata, plotfile)?)
}

fn get_plot_data(opts: &GlobalOptions) -> Result<PlotData, Error> {
    let mut data = load_data(&opts.db_file)?;
    let data = data.get()?;

    let commits = data.sorted_commits();

    let ref series_descs = [
        (Profile::Dev, RebuildType::Full),
        (Profile::Dev, RebuildType::Partial),
        (Profile::Release, RebuildType::Full),
        (Profile::Release, RebuildType::Partial),
    ];
    let mut serieses = Vec::new();
    for series_desc in series_descs {
        let mut new_series = Vec::new();
        for commit in &commits {
            let timings = data.timings.get(commit);
            if let Some(timings) = timings {
                let timings = timings.iter().filter(|t| t.result == BuildResult::Success);
                let timings = timings.filter(|t| t.profile == series_desc.0);
                let timings = timings.filter(|t| t.rebuild_type == series_desc.1);
                let timings = timings.map(|t| t.duration);
                let (count, sum) = timings.fold((0, Duration::default()), |(count, sum), duration| (count + 1, sum + duration));
                if count == 0 {
                    println!("warning: no timings for {}, profile {} type {}", commit.as_ref(), series_desc.0.as_ref(), series_desc.1.as_ref());
                    continue;
                }
                let avg = sum / u32::try_from(count).expect("count should fit in u32");
                let commit = data.commits.get(commit);
                let commit = commit.expect("commit with timing should exists");
                let commit = commit.clone();
                let entry = Entry {
                    commit,
                    duration: avg,
                };
                new_series.push(entry);
            } else {
                println!("warning: no timings for {}", commit.as_ref());
            }
        }

        serieses.push(Series {
            profile: series_desc.0,
            rebuild_type: series_desc.1,
            values: new_series,
        });
    }

    Ok(PlotData(serieses))
}

fn bisect(opts: &GlobalOptions) -> Result<(), Error> {
    let plotdata = get_plot_data(opts)?;
    Ok(bisect::bisect(opts, plotdata)?)
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
    #[display(fmt = "running cargo")]
    Cargo(crate::cargo::Error),
    #[display(fmt = "commit list I/O")]
    CommitListIo(std::io::Error),
    #[display(fmt = "parsing commit")]
    CommitParse(parse_list::ParseListError<crate::commit_list::Error>),
    #[display(fmt = "running gnuplot")]
    GnuPlot(crate::gnuplot::Error),
    #[display(fmt = "bisecting")]
    Bisect(crate::bisect::Error),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::AtomBlob(ref e) => Some(e),
            Error::Git(ref e) => Some(e),
            Error::UnresolvedCommits => None,
            Error::NoCommits => None,
            Error::Cargo(ref e) => Some(e),
            Error::CommitListIo(ref e) => Some(e),
            Error::CommitParse(ref e) => Some(e),
            Error::GnuPlot(ref e) => Some(e),
            Error::Bisect(ref e) => Some(e),
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

impl From<crate::cargo::Error> for Error {
    fn from(e: crate::cargo::Error) -> Error {
        Error::Cargo(e)
    }
}

impl From<crate::gnuplot::Error> for Error {
    fn from(e: crate::gnuplot::Error) -> Error {
        Error::GnuPlot(e)
    }
}

impl From<crate::bisect::Error> for Error {
    fn from(e: crate::bisect::Error) -> Error {
        Error::Bisect(e)
    }
}
