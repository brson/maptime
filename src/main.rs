#![allow(unused)]

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;

mod commit_id;
mod commit_list;
mod data;
mod exec;
mod opts;

fn main() {
    use std::process;
    use std::error::Error;
    use structopt::StructOpt;
    use opts::Options;

    let opts = Options::from_args();

    eprintln!("opts: {:#?}", opts);

    if let Err(e) = exec::run_command(&opts) {
        eprintln!("error: {}", e);

        let mut maybe_source = e.source();
        while let Some(source) = maybe_source {
            eprintln!("source: {}", source);
            maybe_source = source.source();
        }

        process::exit(1);
    }
}
