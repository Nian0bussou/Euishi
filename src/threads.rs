use crate::{movingfn, scrambling, temps_file, utils, CmdsOptions, TimingGuard};
use core::panic;
use std::{io, thread};

pub fn threads_sorting(path: String, opt: CmdsOptions) {
    use CmdsOptions::Move;
    use CmdsOptions::Scramble;

    let dirs: Vec<String> = utils::get_folders(&path);

    let choose = match opt {
        Move { chooseDirs } => chooseDirs,
        Scramble { chooseDirs } => chooseDirs,
        _ => false,
    };

    let dirs = if choose {
        // + addind dirs
        let mut newdirs: Vec<String>;
        loop {
            newdirs = utils::addingDirs(dirs.clone());
            println!("Are the dirs correct (Y/n)");
            let mut str = String::new();
            _ = io::stdin().read_line(&mut str);
            let str = str.trim();
            if str != "n" {
                break;
            }
        }
        newdirs
        // -
    } else {
        dirs
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
            .map(|source| spawn(move || move_stuff(source)))
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
}

pub fn threads_tmps(path: String, printmsg: bool) {
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
