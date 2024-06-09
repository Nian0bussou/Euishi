use lazy_static::lazy_static;
use std::{sync::Mutex, u128};

pub struct Countstruct {
    process: u128,
    succeed: u128,
    failed: u128,
    landscapes: u128,
    squares: u128,
    portraits: u128,
    dir_created: u128,
    tmp_removed: u128,
}

impl Countstruct {
    pub fn new() -> Countstruct {
        Countstruct {
            process: 0,
            succeed: 0,
            failed: 0,
            landscapes: 0,
            squares: 0,
            portraits: 0,
            dir_created: 0,
            tmp_removed: 0,
        }
    }

    pub fn get_process(&self) -> (u128, u128, u128) {
        (self.process, self.succeed, self.failed)
    }
    pub fn get_images_types(&self) -> (u128, u128, u128) {
        (self.landscapes, self.portraits, self.squares)
    }
    pub fn get_dir_coutn(&self) -> u128 {
        self.dir_created
    }
    pub fn get_tmp_count(&self) -> u128 {
        self.tmp_removed
    }

    pub fn propp(&mut self) {
        self.process += 1;
    }
    pub fn sucpp(&mut self) {
        self.succeed += 1;
    }
    pub fn faipp(&mut self) {
        self.failed += 1;
    }
    pub fn landpp(&mut self) {
        self.landscapes += 1;
    }
    pub fn portpp(&mut self) {
        self.portraits += 1;
    }
    pub fn squapp(&mut self) {
        self.squares += 1;
    }

    pub fn dir_countpp(&mut self) {
        self.dir_created += 1;
    }
    pub fn tmp_removedpp(&mut self) {
        self.tmp_removed += 1;
    }
}

lazy_static! {
    pub static ref GLOBAL_COUNTS: Mutex<Countstruct> = Mutex::new(Countstruct::new());
}

// pub fn init_count() {
//     let _ = GLOBAL_COUNTS;
// }
