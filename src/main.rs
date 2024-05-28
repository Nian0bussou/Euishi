use std::thread::sleep;
use std::time::Duration;
use std::{process::exit, thread, time::Instant};
use utils::{exit_msg, get_path};

mod counting;
mod movingfn;
mod scrambling;
mod temps_file;
mod utils;

fn main() {
    let _time = TimingGuard::new();
    counting::init_count();
    sleep(Duration::from_millis(2000));
    let choice = utils::get_choice();
    let path = get_path();
    let subs = utils::get_folders(&path);
    utils::line();

    // multithreading go brrrr
    let handles: Vec<_> = subs
        .into_iter()
        .map(|source| {
            thread::spawn(move || match choice {
                0 => movingfn::move_stuff(source),
                1 => scrambling::scramble(source),
                _ => exit(i32::MAX),
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    temps_file::remove_tmps(&path);

    utils::line();
    exit_msg();
}

struct TimingGuard {
    start: Instant,
}

impl Drop for TimingGuard {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        println!("time: {:?}", duration)
    }
}

impl TimingGuard {
    fn new() -> Self {
        TimingGuard {
            start: Instant::now(),
        }
    }
}
