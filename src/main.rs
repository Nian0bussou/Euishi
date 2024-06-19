#![allow(non_snake_case)]
mod counting;
mod movingfn;
mod scrambling;
mod temps_file;
mod tests;
mod utils;
use crate::utils::line;
use std::{thread, time::Instant};

fn main() {
    let _t = TimingGuard::new();
    let (move_scramble, doRemoveTmps, haveCustomPath) = utils::g_Choices();
    let path: String = utils::get_path(haveCustomPath);
    let printmsg = true;

    threads_sorting(path.clone(), move_scramble);
    if doRemoveTmps {
        threads_tmps(path, printmsg)
    }
    utils::exit_msg();
}

fn threads_sorting(path: String, choice: bool) {
    let handles: Vec<_> = utils::get_folders(&path)
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

fn threads_tmps(path: String, printmsg: bool) {
    println!("removing tmps files");

    let tmp = utils::get_folders(&path);
    let vtmp: Vec<_> = tmp
        .iter()
        .map(|t| {
            let s: String = format!("{}/", t);
            s
        })
        .collect();
    let vvtmp: Vec<String> = vtmp
        .iter()
        .flat_map(|sub| utils::get_folders(sub))
        .collect();
    //let mut vvtmp: Vec<String> = Vec::new();
    //for sub in vtmp {
    //    for s in utils::get_folders(&sub) {
    //        vvtmp.push(s)
    //    }
    //}
    let handles: Vec<_> = vvtmp
        .into_iter()
        .map(|source: String| thread::spawn(move || temps_file::remove_tmps(&source, printmsg)))
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
