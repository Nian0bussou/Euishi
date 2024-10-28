use crate::counting::pcount;
use crate::utils;
use std::fs;
use std::path::Path;
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
            pcount("proc");
            match fs::rename(p, &new_path) {
                Ok(_) => {
                    pcount("succ");
                    utils::scramble_log(true, new_path);
                }
                Err(_) => {
                    pcount("fail");
                    utils::scramble_log(false, new_path);
                }
            }
        }
    }
}
