use crate::{counting::countpp, utils};
use std::{fs, path::Path};
use walkdir::WalkDir;

/// move all files into the root directory provided by 'path'
///
/// # Panics
///
/// Panics if GLOBAL_COUNTS.lock().unwrap() panics;
pub fn scramble(path: String) {
    let path = Path::new(&path);

    for entry in WalkDir::new(path) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => return,
        };
        if entry.file_type().is_file() {
            let p = entry.path();
            let filename = match p.file_name() {
                Some(name) => name,
                None => continue,
            };
            let new_path = path.join(filename);
            countpp("proc");
            match fs::rename(p, &new_path) {
                Ok(_) => {
                    countpp("succ");
                    utils::scramble_log(true, new_path);
                }
                Err(_) => {
                    countpp("fail");
                    utils::scramble_log(false, new_path);
                }
            }
        }
    }
}
