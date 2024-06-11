use std::fs;

use walkdir::WalkDir;

use crate::counting::GLOBAL_COUNTS;

/// # remove temp
/// walk through dir to remove .tmp files
/// avoid using if another program is handling (download, editing a document, etc.)
/// in one of the subdirectory
pub fn remove_tmps(path: &str) {
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if false {
            println!("{}", path.display().to_string())
        };
        if path.is_file() && path.extension().unwrap() == "tmp" {
            let mut guard = GLOBAL_COUNTS.lock().unwrap();
            guard.fieldPP("tmp");
            let _ = fs::remove_file(path);
        }
    }
}
