#![allow(non_snake_case)]
use std::{thread, time::Instant};
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
    let _t = TimingGuard::new();
    let choices = utils::get_choices();
    let path: String = utils::get_path(choices.haveCustomPath);
    counting::init_count();
    threads_sorting(path.clone(), choices.move_scramble);

    if choices.doRemoveTmps {
        threads_tmps(path);
    }
    utils::exit_msg();
}

/// handle the multithreading of
///
/// either : movingfn::move_stuff(source),
/// or     : scrambling::scramble(source)
fn threads_sorting(path: String, choice: bool) {
    let subs: Vec<String> = utils::get_folders(&path);
    let handles: Vec<_> = subs
        .into_iter()
        .map(|source| {
            thread::spawn(move || match choice {
                true => movingfn::move_stuff(source),
                // false => panic!("false in hoice"),
                false => scrambling::scramble(source), // FIXME
            })
        })
        .collect();
    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}

/// go through every directory and each subdirectory recursively
/// to remove .tmp files
///
/// # Panics
///
/// Panics if .
fn threads_tmps(path: String) {
    println!("removing tmps files");
    let subs: Vec<String> = utils::get_folders(&path);
    //
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
    //
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
