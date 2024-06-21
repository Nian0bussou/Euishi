#![allow(non_snake_case)]
mod counting;
mod movingfn;
mod scrambling;
mod temps_file;
mod tests;
mod threads;
mod utils;

use crate::utils::line;
use ::clap::Parser;
use std::time::Instant;

///
/// main function
fn main() {
    let flags = Args::parse();

    let path: String = utils::get_path(flags.path);

    threads::threads_sorting(path.clone(), flags.move_, flags.scramble);
    if flags.removeTmps {
        threads::threads_tmps(path, flags.verbose)
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

#[derive(Parser, Debug)]
pub struct Args {
    // argfs take the first letter of the var name
    // and long version is the var name

    //
    /// will print each file when using removeTmps
    #[arg(short, long)]
    verbose: bool,

    /// sort the files ; if both move & scramble are provided scramble will be used first
    #[arg(short, long)]
    move_: bool,

    /// scramble the files ; if both move & scramble are provided scramble will be used first
    #[arg(short, long)]
    scramble: bool,

    /// remove tmp files
    #[arg(short, long)]
    removeTmps: bool,

    /// provide the path
    #[arg(short, long)]
    path: Option<String>,
}
