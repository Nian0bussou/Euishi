use std::fs;

use walkdir::WalkDir;

use crate::counting::pcount;

pub fn remove_tmps(path: &str) {
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if false {
            println!("{}", path.display().to_string())
        };
        if path.is_file() && path.extension().unwrap() == "tmp" {
            pcount("tmp");
            let _ = fs::remove_file(path);
        }
    }
}
