use {
    clap::Parser,
    flags::{Args, Commands},
    std::time::Instant,
    threads::{threads_sorting, threads_tmps},
    utils::{get_path, line},
};

mod counting;
mod flags;
mod getjson;
mod movingfn;
mod outfile;
mod scrambling;
mod temps_file;
mod tests;
mod threads;
mod utils;

//
/// main fn
pub fn main() {
    let (opt, fpath) = handle_flags();

    use CmdsOptions::Invalid;
    use CmdsOptions::Move;
    use CmdsOptions::Remove;
    use CmdsOptions::Scramble;

    match opt {
        Move { choose_dirs } /*-----*/=> threads_sorting(get_path(fpath), Move { choose_dirs }),
        Scramble { choose_dirs } /*-*/=> threads_sorting(get_path(fpath), Scramble { choose_dirs }),
        Remove { verbose } /*-------*/=> threads_tmps(get_path(fpath), verbose),
        Invalid => (),
    }
}

fn handle_flags() -> (CmdsOptions, Option<String>) {
    use CmdsOptions::{Invalid, Move, Remove, Scramble};

    match &Args::parse().command {
        Some(Commands::Move_ { path, choose_dirs }) => (
            Move {
                choose_dirs: choose_dirs.clone(),
            },
            path.clone(),
        ),
        Some(Commands::Scramble { path, choose_dirs }) => (
            Scramble {
                choose_dirs: choose_dirs.clone(),
            },
            path.clone(),
        ),
        Some(Commands::Remove { path, verbose }) => (Remove { verbose: *verbose }, path.clone()),
        //
        _ => (Invalid, None),
    }
}

#[derive(Clone)]
enum CmdsOptions {
    Move { choose_dirs: Option<String> },
    Scramble { choose_dirs: Option<String> },
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
