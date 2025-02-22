use crate::movingfn;
use crate::scrambling;
use crate::temps_file;
use crate::utils;
use crate::utils::exit_msg;
use crate::CmdsOptions;
use crate::TimingGuard;
use core::panic;
use movingfn::move_stuff;
use scrambling::scramble;
use std::thread;

pub fn t_sorting(path: String, opt: CmdsOptions) {
    use CmdsOptions::Move;
    use CmdsOptions::Scramble;

    let _t = TimingGuard::new();

    match opt {
        Move { .. } => {
            move_stuff(path)
        }
        Scramble { .. } => scramble(path),
        _ => panic!("not supposed to get here"),
    };

    exit_msg();
}

pub fn t_tmps(path: String, printmsg: bool) {
    println!("removing tmps files");

    let tmpd: Vec<String> = utils::get_folders(&path)
        .iter()
        .map(|t| {
            let s: String = format!("{}/", t);
            s
        })
        .collect::<Vec<String>>()
        .iter()
        .flat_map(|sub| utils::get_folders(sub))
        .collect();

    let _t = TimingGuard::new();
    let handles: Vec<_> = tmpd
        .into_iter()
        .map(|source: String| thread::spawn(move || temps_file::remove_tmps(&source, printmsg)))
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
