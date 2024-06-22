#![allow(non_snake_case)]
mod counting;
mod flags;
mod movingfn;
mod scrambling;
mod temps_file;
mod tests;
mod threads;
mod utils;

use crate::utils::line;
use clap::Parser;
use flags::Args;
use flags::Commands;
use std::time::Instant;

/// main function
pub fn main() {
    let flags = Args::parse();

    let mut moving: bool = false;
    let mut scramble: bool = false;
    let mut remove: bool = false;
    let mut verboses: bool = false;
    let mut flagpath: Option<String> = None;

    match &flags.command {
        Some(Commands::Move_ { path }) => {
            flagpath = path.clone();
            moving = true
        }
        Some(Commands::Scramble { path }) => {
            flagpath = path.clone();
            scramble = true
        }
        Some(Commands::Remove { path, verbose }) => {
            flagpath = path.clone();
            remove = true;
            verboses = *verbose;
        }
        Some(Commands::Skibiditoiletrizzinohiofrfrbrainrot) => (),
        None => (),
    }

    let path: String = utils::get_path(flagpath);

    if moving || scramble {
        threads::threads_sorting(path.clone(), moving, scramble);
    }
    if remove {
        threads::threads_tmps(path, verboses)
    }
    utils::exit_msg();
}

//
//
//
//
//

struct TimingGuard {
    start: Instant,
}

impl Drop for TimingGuard {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        println!("time taken: {:?}", duration);
        line();
    }
}
impl TimingGuard {
    fn new() -> Self {
        TimingGuard {
            start: Instant::now(),
        }
    }
}
