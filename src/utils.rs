use core::panic;
use std::{
    collections::HashSet,
    env::{args, consts::OS},
    fs,
    path::{Path, PathBuf},
    u8,
};

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
        _ => panic!(/* mainly for debug */
            "invalid option ; got : {} , expected : {{\"land\", \"square\", \"portrait\", \"video\"}}", 
            dest_type 
        ),
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
        _ if args.len() == 4 => path = args[3].clone(),
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
// bool cuz no need for more options yet
pub fn get_choice() -> bool {
    let args: Vec<_> = args().collect();
    if args.len() < 3 {
        return false;
    }
    match &args[2].parse::<u8>() {
        Ok(n) => match n {
            0 => return false,
            1 => return true,
            _ => {
                println!("invalid option");
                std::process::exit(1984);
            }
        },
        Err(_) => {
            eprintln!("could not get choice");
            std::process::exit(19198484);
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
                            folders.insert(name_str.to_string()); // nesting hell 🏳️‍⚧️
                        }
                    }
                }
            }
        }
    }
    let mut vecs: Vec<String> = folders.into_iter().collect();
    vecs.sort();
    vecs.into_iter()
        .map(|folder| format!("{}{}", directory, folder))
        .collect()
}

pub fn exit_msg() {
    let guard = GLOBAL_COUNTS.lock().unwrap();
    let (p, s, f) = guard.get_process();
    let (la, po, sq) = guard.get_images_types();
    let ds = guard.get_dir_coutn();
    let tr = guard.get_tmp_count();
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
    println!("tmp removed : {}", tr);
    line();
}

pub fn scramble_log(okerr: bool, f: PathBuf) {
    let str_f = f.display().to_string();
    match okerr {
        true => println!("       moved : {}", str_f),
        false => println!("dit not move : {}", str_f),
    }
}
