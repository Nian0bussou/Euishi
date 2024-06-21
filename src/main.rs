#![allow(non_snake_case)]
mod counting;
mod movingfn;
mod scrambling;
mod temps_file;
mod tests;
mod utils;
use crate::utils::line;
use std::{io, thread, time::Instant};

/*$$$$$$$                        /$$
| $$__  $$                      | $$
| $$  \ $$ /$$   /$$  /$$$$$$$ /$$$$$$
| $$$$$$$/| $$  | $$ /$$_____/|_  $$_/
| $$__  $$| $$  | $$|  $$$$$$   | $$
| $$  \ $$| $$  | $$ \____  $$  | $$ /$$
| $$  | $$|  $$$$$$/ /$$$$$$$/  |  $$$$/
|__/  |__/ \______/ |_______/    \___/

 /$$      /$$
| $$$    /$$$
| $$$$  /$$$$  /$$$$$$  /$$    /$$  /$$$$$$
| $$ $$/$$ $$ /$$__  $$|  $$  /$$/ /$$__  $$
| $$  $$$| $$| $$  \ $$ \  $$/$$/ | $$$$$$$$
| $$\  $ | $$| $$  | $$  \  $$$/  | $$_____/
| $$ \/  | $$|  $$$$$$/   \  $/   |  $$$$$$$
|__/     |__/ \______/     \_/     \______*/

fn main() {
    let (move_scramble, doRemoveTmps, haveCustomPath) = utils::g_Choices();
    let path: String = utils::get_path(haveCustomPath);
    let printmsg = true;

    threads_sorting(path.clone(), move_scramble);
    if doRemoveTmps {
        threads_tmps(path, printmsg)
    }
    utils::exit_msg();
}

#[allow(unreachable_code)]
fn threads_sorting(path: String, choice: bool) {
    let dirs: Vec<String> = utils::get_folders(&path);
    // removed dirs
    let mut newdirs: Vec<String>;
    loop {
        newdirs = utils::removedDirs(dirs.clone());
        println!("Are the dirs correct (Y/n)");
        let mut str = String::new();
        _ = io::stdin().read_line(&mut str);
        let str = str.trim();
        if str != "n" {
            break;
        }
    }
    let dirs = newdirs;

    let _t = TimingGuard::new();
    let handles: Vec<_> = dirs
        .into_iter()
        .map(|source| {
            thread::spawn(move || match choice {
                true => movingfn::move_stuff(source),
                false => scrambling::scramble(source),
            })
        })
        .collect();
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

    let _t = TimingGuard::new();
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
