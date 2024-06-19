#[allow(unused)]
use crate::temps_file;
#[allow(unused)]
use crate::utils;
#[allow(unused)]
use std::thread;

// test made to crash
#[test]
fn testThreadsTmps() {
    println!("removing tmps files");

    let path = utils::get_path(false);
    let printmsg = false;

    let tmp: Vec<String> = utils::get_folders(&path)
        .iter()
        .map(|t| {
            let s: String = format!("{}/", t);
            s
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|sub| utils::get_folders(&sub).into_iter().map(|s| s).collect())
        .collect::<Vec<_>>();
    let handles: Vec<_> = tmp
        .into_iter()
        .map(|source: String| thread::spawn(move || temps_file::remove_tmps(&source, printmsg)))
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
