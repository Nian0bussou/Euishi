use std::collections::HashSet;
use std::env::args;
use std::env::consts::OS;
use std::fs;
use std::path::{Path, PathBuf};

use std::u8;

use term_size::dimensions;

use crate::counting::GLOBAL_COUNTS;

pub fn file_output(source: &str, dest_path: &str, color_name: &str, dest_type: &str) {
    let reset = "\x1b[0m";
    let color = match color_name {
        "red" => "\x1b[31m",
        "blue" => "\x1b[34m",
        "green" => "\x1b[32m",
        "cyan" => "\x1b[36m",
        "magenta" => "\x1b[35m",
        "purple" => "\x1b[35;1m",
        "grey" => "\x1b[37m",
        "yellow" => "\x1b[33m",
        _ => reset,
    };

    let special = match dest_type {
        "land" => "━",
        "square" => "■",
        "portrait" => "┃",
        "video" => "▶",
        _ => "",
    };

    let tab = "\t";
    let path = Path::new(&source);
    let parentdir = match path.parent() {
        Some(s) => s.display().to_string(),
        None => "".to_string(),
    };
    let padded_dir_str = format!("{:<80}", parentdir);
    println!(
        "{}{}{}{}{}{}{}{}{}",
        tab, color, special, tab, padded_dir_str, "<|====|>", tab, dest_path, reset
    )
}

pub fn error_maxxing() {
    line();
    println!("\x1b[31mERROR\x1b[0m");
    line();
}

pub fn line() {
    let (w, _) = dimensions().unwrap();
    println!("{}", "_".repeat(w));
}

pub fn get_path() -> String {
    let path: String;
    let args: Vec<_> = args().collect();
    match OS {
        _ if args.len() == 3 => path = args[2].clone(),
        "windows" => path = "D:/grapper/".to_owned(),
        "linux" => path = "/mnt/d/grapper/".to_owned(),
        _ => panic!("cant get path"),
    }
    if !(Path::new(&path).exists()) {
        panic!("directory not found");
    }
    path
}

// returns choice whether to (0) sort or (1) scramble
pub fn get_choice() -> u8 {
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        return 0;
    }
    match &args[1].parse::<u8>() {
        Ok(n) => match n {
            0 => return 0,
            1 => return 1,
            _ => {
                println!("invalid option");
                return u8::MAX;
            }
        },
        Err(_) => {
            eprintln!("could not get choice");
            std::process::exit(1);
        }
    }
}

pub fn get_folders(directory: &str) -> Vec<String> {
    let mut folders: HashSet<String> = HashSet::new();
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        if path.is_dir() {
                            folders.insert(name_str.to_string()); // nesting hell
                        }
                    }
                }
            }
        }
    }
    let mut vecs: Vec<String> = folders.into_iter().collect();
    vecs.sort();
    return vecs
        .into_iter()
        .map(|folder| format!("{}{}", directory, folder))
        .collect();
}

pub fn exit_msg() {
    let guard = GLOBAL_COUNTS.lock().unwrap();
    let (p, s, f) = guard.get_process();
    let (la, po, sq) = guard.get_images_types();
    let ds = guard.get_dir_coutn();
    line();
    println!("Finished,");
    println!("count     : {}", p);
    println!("succeeded : {}", s);
    println!("failed    : {}", f);
    line();
    println!("types;");
    println!("land: {}", la);
    println!("port: {}", po);
    println!("squa: {}", sq);
    line();
    println!("Dir created : {}", ds);
    line();
}

pub fn scramble_log(okerr: bool, f: PathBuf) {
    let str_f = f.display().to_string();
    match okerr {
        true => println!("       moved : {}", str_f),
        false => println!("dit not move : {}", str_f),
    }
}
