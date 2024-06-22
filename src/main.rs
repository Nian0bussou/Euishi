#![allow(non_snake_case)]
mod counting;
mod flags;
mod movingfn;
mod outfile;
mod scrambling;
mod temps_file;
mod tests;
mod threads;
mod utils;

use clap::Parser;
use flags::{Args, Commands};
use std::time::Instant;
use utils::line;

pub fn main() {
    let (opt, fpath, verboses) = handleFlags();
    let path: String = utils::get_path(fpath);

    use CmdsOptions::Invalid;
    use CmdsOptions::Move;
    use CmdsOptions::Remove;
    use CmdsOptions::Scramble;

    match opt {
        Move => threads::threads_sorting(path.clone(), Move),
        Scramble => threads::threads_sorting(path.clone(), Scramble),
        Remove => threads::threads_tmps(path, verboses),
        Invalid => (),
    }
    utils::exit_msg();
}

enum CmdsOptions {
    Move,
    Scramble,
    Remove,
    Invalid,
}

fn handleFlags() -> (CmdsOptions, Option<String>, bool) {
    use CmdsOptions::Invalid;
    use CmdsOptions::Move;
    use CmdsOptions::Remove;
    use CmdsOptions::Scramble;

    match &Args::parse().command {
        Some(Commands::Move_ { path })/*______________________*/ => (Move/*_____*/, path.clone(), false),
        Some(Commands::Scramble { path })/*___________________*/ => (Scramble/*_*/, path.clone(), false),
        Some(Commands::Remove { path, verbose })/*____________*/ => (Remove/*___*/, path.clone(), *verbose),
        Some(Commands::Skibiditoiletrizzinohiofrfrbrainrot)/*_*/ => (Invalid/*__*/, None/*____*/, false),
        None /*_______________________________________________*/ => (Invalid/*__*/, None/*____*/, false),
    }
}

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
