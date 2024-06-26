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
use {
    clap::Parser,
    flags::{Args, Commands},
    std::time::Instant,
    threads::{threads_sorting, threads_tmps},
    utils::{exit_msg, get_path, line},
};
//
/// main fn
pub fn main() {
    let (opt, fpath) = handleFlags();

    use CmdsOptions::Invalid;
    use CmdsOptions::Move;
    use CmdsOptions::Remove;
    use CmdsOptions::Scramble;

    match opt {
        Move { chooseDirs } /*-----*/=> threads_sorting(get_path(fpath), Move { chooseDirs }),
        Scramble { chooseDirs } /*-*/=> threads_sorting(get_path(fpath), Scramble { chooseDirs }),
        Remove { verbose } /*------*/=> threads_tmps(get_path(fpath), verbose),
        Invalid => (),
    }

    exit_msg();
}

fn handleFlags() -> (CmdsOptions, Option<String>) {
    use CmdsOptions::{Invalid, Move, Remove, Scramble};

    match &Args::parse().command {
        Some(Commands::Move_ { path, chooseDirs }) => (
            Move {
                chooseDirs: *chooseDirs,
            },
            path.clone(),
        ),
        Some(Commands::Scramble { path, chooseDirs }) => (
            Scramble {
                chooseDirs: *chooseDirs,
            },
            path.clone(),
        ),
        Some(Commands::Remove { path, verbose }) => (Remove { verbose: *verbose }, path.clone()),
        //
        _ => (Invalid, None),
    }
}

enum CmdsOptions {
    Move { chooseDirs: bool },
    Scramble { chooseDirs: bool },
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
    _ = handleFlags();
}
