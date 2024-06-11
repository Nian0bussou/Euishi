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

    /// # increase by 1 a field in [Countstruct].
    /// select which one by providing one of the following [`&str`]
    /// "proc", "succ", "fail", "land", "port", "squa", "vide", "dir_", "tmp_",
    /// # Panics
    /// Panics if the provided [`&str`].
    pub fn fieldPP(&mut self, field: &str) {
        match field {
            "proc" => self.process += 1,
            "succ" => self.succeed += 1,
            "fail" => self.failed += 1,
            "land" => self.landscapes += 1,
            "port" => self.portraits += 1,
            "squa" => self.squares += 1,
            "vide" => self.video += 1,
            "dir_" => self.dir_created += 1,
            "tmp_" => self.tmp_removed += 1,
            _ => panic!("invalid field"),
        }
    }
}

lazy_static! {
    pub static ref GLOBAL_COUNTS: Mutex<Countstruct> = Mutex::new(Countstruct::new());
}

pub fn init_count() {
    let _ = GLOBAL_COUNTS;
}
