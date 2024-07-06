use crate::{
    movingfn, scrambling, temps_file,
    utils::{self, exit_msg},
    CmdsOptions, TimingGuard,
};
use core::panic;
use std::thread;

pub fn t_sorting(path: String, opt: CmdsOptions) {
    use CmdsOptions::Move;
    use CmdsOptions::Scramble;

    let mut dirs: Vec<String> = utils::get_folders(&path);
    dirs.retain(|d| !d.contains("_______________"));
    let dirs = dirs;

    let choose = match opt.clone() {
        Move { choose_dirs } => choose_dirs,
        Scramble { choose_dirs } => choose_dirs,
        _ => None,
    };

    let dirs = match choose.clone() {
        Some(choose) => match utils::adding_dirs(choose) {
            Ok(e) => e,
            Err(_) => dirs,
        },
        None => dirs,
    };

    let _t = TimingGuard::new();

    use movingfn::move_stuff;
    use scrambling::scramble;
    use thread::spawn;
    // + threads -->
    let handles: Vec<_> = match opt {
        Move { .. } => dirs
            .clone()
            .into_iter()
            .map(|source| {
                spawn({
                    let value = choose.clone();
                    move || move_stuff(source, value)
                })
            })
            .collect(),
        Scramble { .. } => dirs
            .clone()
            .into_iter()
            .map(|source| spawn(move || scramble(source)))
            .collect(),
        _ => panic!("not supposed to get here"),
    };
    // -
    for handle in handles {
        handle.join().unwrap();
    }

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
