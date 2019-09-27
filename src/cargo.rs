use std::path::PathBuf;
use std::fs::OpenOptions;
use std::time::{Instant, Duration};
use chrono::{DateTime, Utc};
use std::error::Error as StdError;
use crate::data::Profile;
use std::path::Path;
use crate::data::BuildResult;
use crate::data::Timing;
use crate::data::RebuildType;
use std::process::Command;
use std::process::ExitStatus;

pub struct BuildResultPair {
    pub full: Timing,
    pub partial: Option<Timing>,
    pub touched: Option<PathBuf>,
}

pub fn time_build(path: &Path, profile: Profile) -> Result<BuildResultPair, Error> {
    prime_toolchain(path)?;
    cargo_clean(path)?;
    cargo_fetch(path)?;

    let full_result = cargo_time_build(path, profile, RebuildType::Full)?;
    if full_result.result == BuildResult::Failure {
        return Ok(BuildResultPair {
            full: full_result,
            partial: None,
            touched: None,
        });
    }

    let touched = touch_something(path)?;
    let partial_result = cargo_time_build(path, profile, RebuildType::Partial)?;

    Ok(BuildResultPair {
        full: full_result,
        partial: Some(partial_result),
        touched: Some(touched),
    })
}

fn cargo_clean(path: &Path) -> Result<(), Error> {
    println!("running `cargo clean`");

    let mut cmd = Command::new("cargo");
    let cmd = cmd
        .current_dir(path)
        .arg("clean");

    let status = cmd.status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::CargoClean)
    }
}

fn cargo_fetch(path: &Path) -> Result<(), Error> {
    println!("running `cargo fetch`");

    let mut cmd = Command::new("cargo");
    let cmd = cmd
        .current_dir(path)
        .arg("fetch");

    let status = cmd.status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::CargoFetch)
    }
}

fn prime_toolchain(path: &Path) -> Result<(), Error> {
    println!("running `rustc -V` to prime the toolchain");

    let mut cmd = Command::new("rustc");
    let cmd = cmd
        .current_dir(path)
        .arg("-V");

    let status = cmd.status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::PrimeToolchain)
    }
}

fn cargo_time_build(path: &Path, profile: Profile, rebuild_type: RebuildType) -> Result<Timing, Error> {
    println!("running `cargo build` for {} profile, {} rebuild", profile.as_ref(), rebuild_type.as_ref());

    let mut cmd = Command::new("cargo");
    let mut cmd = cmd
        .current_dir(path)
        .env("CARGO_BUILD_PIPELINING", "true")
        .arg("build");

    if profile == Profile::Release {
        cmd = cmd.arg("--release");
    }

    let start_date = Utc::now();
    let start = Instant::now();

    let status = cmd.status()?;
    
    let dur = start.elapsed();

    let res = if status.success() { BuildResult::Success } else { BuildResult::Failure };

    Ok(Timing {
        profile,
        rebuild_type,
        start: start_date,
        duration: dur,
        result: res,
    })
}

fn touch_something(path: &Path) -> Result<PathBuf, Error> {
    let candidates = ["src/lib.rs", "src/main.rs"];

    for candidate in &candidates {
        let candidate = PathBuf::from(candidate);
        let path = path.join(&candidate);
        let file = OpenOptions::new().append(true).open(path);
        match file {
            Ok(_) => return Ok(PathBuf::from(candidate)),
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(Error::Io(e)),
        }
    }

    Err(Error::CantTouch)
}

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "I/O error running cargo")]
    Io(std::io::Error),
    #[display(fmt = "cargo clean failed")]
    CargoClean,
    #[display(fmt = "cargo fetch failed")]
    CargoFetch,
    #[display(fmt = "unable to find file to touch for partial rebuild")]
    CantTouch,
    #[display(fmt = "priming toolchain")]
    PrimeToolchain,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(ref e) => Some(e),
            Error::CargoClean => None,
            Error::CargoFetch => None,
            Error::CantTouch => None,
            Error::PrimeToolchain => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}
