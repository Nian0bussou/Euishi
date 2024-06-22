#![allow(non_snake_case)]
mod counting;
mod flags;
mod movingfn;
mod scrambling;
mod temps_file;
mod tests;
mod threads;
mod utils;

use clap::Parser;
use flags::{Args, Commands};
use std::time::Instant;
use utils::line;

//
//
//
//
//

pub fn main() {
    let (opt, fpath, verboses) = handleFlags();
    let path: String = utils::get_path(fpath);

    use DestroyerOfWorlds::Invalid;
    use DestroyerOfWorlds::Move;
    use DestroyerOfWorlds::Remove;
    use DestroyerOfWorlds::Scramble;

    match opt {
        Move => threads::threads_sorting(path.clone(), Move),
        Scramble => threads::threads_sorting(path.clone(), Scramble),
        Remove => threads::threads_tmps(path, verboses),
        Invalid => (),
    }
    utils::exit_msg();
}

//
//
//
//
//

enum DestroyerOfWorlds {
    Move,
    Scramble,
    Remove,
    Invalid,
}

fn handleFlags() -> (DestroyerOfWorlds, Option<String>, bool) {
    use DestroyerOfWorlds::Invalid;
    use DestroyerOfWorlds::Move;
    use DestroyerOfWorlds::Remove;
    use DestroyerOfWorlds::Scramble;

    match &Args::parse().command {
        Some(Commands::Move_ { path }) => {
            //
            (Move, path.clone(), false)
        }
        Some(Commands::Scramble { path }) => {
            //
            (Scramble, path.clone(), false)
        }

        Some(Commands::Remove { path, verbose }) => {
            //
            (Remove, path.clone(), *verbose)
        }

        Some(Commands::Skibiditoiletrizzinohiofrfrbrainrot) => {
            //
            (Invalid, None, false)
        }

        None => (Invalid, None, false),
    }
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
