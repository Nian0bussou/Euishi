use std::{env::args, process::exit, thread, time::Instant};
mod counting;
mod movingfn;
mod scrambling;
mod temps_file;
mod utils;

////////////////////////////////////////////////
//                                            //
//  I WILL POST CRINGE IN #GENERAL, MACHINE   //
//                                            //
////////////////////////////////////////////////

fn main() {
    let _t: TimingGuard = TimingGuard::new();
    let path: String = utils::get_path();
    counting::init_count();
    threads_brr(path.clone());
    let args: Vec<_> = args().collect();
    if args.len() == 2 {
        if args[1] == "1" {
            threads_tmps(path);
        }
    }
    utils::exit_msg();
}

fn threads_brr(path: String) {
    let subs: Vec<String> = utils::get_folders(&path);
    let choice: u8 = utils::get_choice();
    let handles: Vec<_> = subs
        .into_iter()
        .map(|source| {
            thread::spawn(move || match choice {
                0 => movingfn::move_stuff(source),
                1 => scrambling::scramble(source),
                _ => exit(2532136),
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
