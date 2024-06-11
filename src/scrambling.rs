use crate::{counting::GLOBAL_COUNTS, utils};
use std::{fs, path::Path};
use walkdir::WalkDir;

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
            let mut guard = GLOBAL_COUNTS.lock().unwrap();
            guard.fieldPP("proc");
            match fs::rename(p, &new_path) {
                Ok(_) => {
                    guard.fieldPP("succ");
                    utils::scramble_log(true, new_path);
                }
                Err(_) => {
                    let mut guard = GLOBAL_COUNTS.lock().unwrap();
                    guard.fieldPP("fail");
                    utils::scramble_log(false, new_path);
                }
            }
        }
    }
}
