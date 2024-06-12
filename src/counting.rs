use core::panic;
use lazy_static::lazy_static;
use std::{sync::Mutex, u128};

pub struct Countstruct {
    process: u128,
    succeed: u128,
    failed: u128,
    landscapes: u128,
    squares: u128,
    portraits: u128,
    video: u128,
    dir_created: u128,
    tmp_removed: u128,
}

impl Countstruct {
    /// Creates a new [`Countstruct`].
    /// :
    /// Self {
    ///     process: 0,
    ///     succeed: 0,
    ///     failed: 0,
    ///     landscapes: 0,
    ///     squares: 0,
    ///     portraits: 0,
    ///     video: 0,
    ///     dir_created: 0,
    ///     tmp_removed: 0,
    /// }
    pub fn new() -> Self {
        Self {
            process: 0,
            succeed: 0,
            failed: 0,
            landscapes: 0,
            squares: 0,
            portraits: 0,
            video: 0,
            dir_created: 0,
            tmp_removed: 0,
        }
    }

    /// Returns the get process of this [Countstruct]:
    /// { process, succeed, failed, landscapes, squares, portraits, video, dir_created, tmp_removed }
    pub fn get_process(&self) -> (u128, u128, u128, u128, u128, u128, u128, u128, u128) {
        (
            self.process,
            self.succeed,
            self.failed,
            self.landscapes,
            self.squares,
            self.portraits,
            self.video,
            self.dir_created,
            self.tmp_removed,
        )
    }
}

lazy_static! {
    pub static ref GLOBAL_COUNTS: Mutex<Countstruct> = Mutex::new(Countstruct::new());
}

/// start a global ref to [`Countstruct`]
pub fn init_count() {
    let _ = GLOBAL_COUNTS;
}
/// # increase by 1 a field in [GLOBAL_COUNTS] which is a [Countstruct].
/// select which one by providing one of the following [`&str`]
/// "proc", "succ", "fail", "land", "port", "squa", "vide", "dir", "tmp",
/// # Panics
/// Panics if the provided [`&str`].
pub fn countpp(field: &str) {
    let mut c = GLOBAL_COUNTS.lock().unwrap();
    match field {
        "proc" => c.process += 1,
        "succ" => c.succeed += 1,
        "fail" => c.failed += 1,
        "land" => c.landscapes += 1,
        "port" => c.portraits += 1,
        "squa" => c.squares += 1,
        "vide" => c.video += 1,
        "dir" => c.dir_created += 1,
        "tmp" => c.tmp_removed += 1,
        _ => panic!("invalid field"),
    }
}
