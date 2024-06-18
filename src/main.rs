#![allow(non_snake_case)]
use std::{thread, time::Instant};

use crate::utils::line;
mod counting;
mod movingfn;
mod scrambling;
mod temps_file;
mod utils;

//////////////////////////////////////
//                                  //
//  I WILL POST CRINGE IN #GENERAL  //
//                                  //
//////////////////////////////////////

fn main() {
    let _t = TimingGuard::new();
    let (move_scramble, doRemoveTmps, haveCustomPath) = utils::get_choices().getAttrs();
    let path: String = utils::get_path(haveCustomPath);

    threads_sorting(path.clone(), move_scramble);
    threads_tmps(path, doRemoveTmps);
    utils::exit_msg();
}

fn threads_sorting(path: String, choice: bool) {
    let subs: Vec<String> = utils::get_folders(&path);
    let handles: Vec<_> = subs
        .into_iter()
        .map(|source| {
            thread::spawn(move || match choice {
                true => movingfn::move_stuff(source),
                false => scrambling::scramble(source),
            })
        })
        .collect();

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}

fn threads_tmps(path: String, on: bool) {
    if !on {
        return;
    }
    println!("removing tmps files");
    let subs: Vec<String> = utils::get_folders(&path);
    let tmp_subs: Vec<String> = subs
        .into_iter()
        .map(|t| {
            let s = format!("{}/", t);
            s
        })
        .collect();

    let subsub: Vec<String> = tmp_subs
        .into_iter()
        .map(|sub| {
            let ts = utils::get_folders(&sub);
            ts.into_iter().map(|t| t).collect()
        })
        .collect();
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
