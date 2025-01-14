use clap::Parser;
use flags::{Args, Commands};
use std::time::Instant;
use threads::t_sorting;
use threads::t_tmps;
use utils::get_path;
use utils::line;

mod counting;
mod flags;
mod movingfn;
mod outfile;
mod scrambling;
mod temps_file;
mod tests;
mod threads;
mod utils;

pub fn main() {
    let (opt, fpath) = handle_flags();

    let gfpaht = get_path(fpath);

    match opt {
        CmdsOptions::Move => t_sorting(gfpaht, CmdsOptions::Move),
        CmdsOptions::Scramble => t_sorting(gfpaht, CmdsOptions::Scramble),
        CmdsOptions::Remove { verbose } => t_tmps(gfpaht, verbose),
        CmdsOptions::Invalid => {}
    }
}

fn handle_flags() -> (CmdsOptions, Option<String>) {
    use CmdsOptions::{Invalid, Move, Remove, Scramble};

    match &Args::parse().command {
        Some(Commands::Move_ { path }) => (Move, path.clone()),
        Some(Commands::Scramble { path }) => (Scramble, path.clone()),
        Some(Commands::Remove { path, verbose }) => (Remove { verbose: *verbose }, path.clone()),
        _ => (Invalid, None),
    }
}

#[derive(Clone)]
enum CmdsOptions {
    Move,
    Scramble,
    Remove { verbose: bool },
    Invalid,
}

struct TimingGuard {
    start: Instant,
}
impl TimingGuard {
    fn new() -> Self {
        TimingGuard {
            start: Instant::now(),
        }
    }
}
impl Drop for TimingGuard {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        line();
        println!("time taken: {:?}", duration);
    }
}

#[test]
fn optionsflags() {
    _ = handle_flags();
}
