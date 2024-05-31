use std::{thread, time::Instant};
mod counting;
mod movingfn;
mod scrambling;
mod temps_file;
mod utils;

fn main() {
    let _a: TimingGuard = TimingGuard::new();
    let path: String = utils::get_path();
    counting::init_count();
    multithreading_handles_bangles(path.clone());
    threads_tmps(path);
    utils::exit_msg();
}

fn multithreading_handles_bangles(path: String) {
    let subs: Vec<String> = utils::get_folders(&path);
    let choice: bool = utils::get_choice();
    let handles: Vec<_> = subs
        .into_iter()
        .map(|source| {
            thread::spawn(move || match choice {
                false => movingfn::move_stuff(source),
                true => scrambling::scramble(source),
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

fn threads_tmps(path: String) {
    println!("removing tmps files");
    let subs: Vec<String> = utils::get_folders(&path);

    let mut tmp_subs: Vec<String> = Vec::new();
    for t in subs {
        let s = format!("{}/", t);
        tmp_subs.push(s)
    }
    let mut subsub: Vec<String> = Vec::new();
    for sub in tmp_subs {
        let tmp_subs = utils::get_folders(&sub);
        for t in tmp_subs {
            subsub.push(t);
        }
    }

    let handles: Vec<_> = subsub
        .into_iter()
        .map(|source: String| thread::spawn(move || temps_file::remove_tmps(&source)))
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
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
