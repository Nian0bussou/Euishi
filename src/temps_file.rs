use std::fs;

use walkdir::WalkDir;

pub fn remove_tmps(path: &str) {
    println!("removing tmps files");
    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap() == "tmp" {
                    let _ = fs::remove_file(path);
                };
            }
            Err(_) => (),
        }
    }
}
