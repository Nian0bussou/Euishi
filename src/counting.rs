use lazy_static::lazy_static;
use std::{sync::Mutex, u128};

lazy_static! {
    pub static ref PROCESSED: Mutex<u128> = Mutex::new(0);
    pub static ref LANDSCAPES: Mutex<u128> = Mutex::new(0);
    pub static ref SQUARES: Mutex<u128> = Mutex::new(0);
    pub static ref PORTRAITS: Mutex<u128> = Mutex::new(0);
    pub static ref VIDEO: Mutex<u128> = Mutex::new(0);
}

pub fn pcount(field: &str) {
    match field {
        "proc" => {
            let mut c = PROCESSED.lock().unwrap();
            *c += 1;
        }
        "land" => {
            let mut c = LANDSCAPES.lock().unwrap();
            *c += 1;
        }
        "port" => {
            let mut c = PORTRAITS.lock().unwrap();
            *c += 1;
        }
        "squa" => {
            let mut c = SQUARES.lock().unwrap();
            *c += 1;
        }
        "vide" => {
            let mut c = VIDEO.lock().unwrap();
            *c += 1;
        }
        _ => (),
    }
}

pub fn g_count() -> (u128, u128, u128, u128, u128) {
    let p = PROCESSED.lock().unwrap();
    let l = LANDSCAPES.lock().unwrap();
    let s = SQUARES.lock().unwrap();
    let o = PORTRAITS.lock().unwrap();
    let v = VIDEO.lock().unwrap();
    (*p, *l, *s, *o, *v)
}

#[test]
fn proc() {
    _ = g_count();
}

#[test]
fn tpcount() {
    _ = pcount("proc");
    _ = pcount("land");
    _ = pcount("port");
    _ = pcount("squa");
    _ = pcount("vide");
    _ = pcount("auishi");
}
