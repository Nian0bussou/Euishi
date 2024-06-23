#![allow(non_snake_case)]
use crate::counting::get_process;
use crate::outfile::makeOutput;
use std::{
    collections::HashSet,
    env::consts::OS,
    fs, io, panic,
    path::{Path, PathBuf},
};
//use term_size::dimensions;

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
        "white" => "\x1b[0m",
        _ => panic!("invalid option ; got {}", color_name),
    };
    let special = match dest_type {
        "land" => "━",
        "square" => "■",
        "portrait" => "┃",
        "video" => "▶",
        "error" => "❌",
        _ => panic!("invalid option ; got {}", dest_type),
    };

    _ = source; // avoid rust complaining the var is not used

    //let path = Path::new(&source);
    //let padded_dir_str = format!("{:<35}", path.display());
    //println!(
    //        "\t{}{}\t{} <|====|> {}{}",
    //        color, special, padded_dir_str, dest_path, reset
    //    )
    println!("\t{color}{special}\t{dest_path}{reset}")
}

pub fn errorPrint(err: String) {
    line();
    let red = "\x1b[31m";
    let white = "\x1b[0m";
    println!("{}ERROR{}: {}", red, err, white);
    line();
}

pub fn line() {
    //let (w, _) = dimensions().unwrap();
    let w = 15;
    println!("{}", "_".repeat(w));
}

pub fn get_path(givenpath: Option<String>) -> String {
    let path = match givenpath {
        Some(p) => p,
        None => match OS {
            "windows" => "D:/grapper/".to_owned(),
            "linux" => "/mnt/d/grapper/".to_owned(),
            _ => panic!("cant get path"),
        },
    };
    assert!(Path::new(&path).exists(), "directory not found");

    path
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
    let (p, l, s, o, v) = get_process();

    let msg = format!(
        "\
_________________    \n\
|=|             |    \n\
| |Finished     |    \n\
| |=============|    \n\
|   count       : {} \n\
|=|             |    \n\
| |Types        |    \n\
| |=============|    \n\
|   landscape   : {} \n\
|   portrait    : {} \n\
|   square      : {} \n\
|   video       : {} \n\
|---------------|    \n\
                     \n\
",
        p, l, o, s, v
    );
    if !(p == 0 && l == 0 && s == 0 && o == 0 && v == 0) {
        println!("{}", msg);
    }

    if let Err(_) = makeOutput(msg) {
        println!("couldn't make output file")
    };
}

pub fn scramble_log(okerr: bool, f: PathBuf) {
    let f = f.display().to_string();
    let truemsg = format!("        moved : {}", f);
    let falsems = format!("dit not moved : {}", f);
    match okerr {
        true => println!("{}", truemsg),
        false => println!("{}", falsems),
    }
}

/// remove dirs from [Vec<String>]
///
pub fn removedDirs(mut dirs: Vec<String>) -> Vec<String> {
    //                           15x _
    dirs.retain(|d| !d.contains("_______________"));

    let mut added: Vec<String> = Vec::new();
    loop {
        if dirs.is_empty() {
            break;
        };

        // display vec
        line();
        let mut idx = 0;
        for dir in &dirs {
            println!("{:>14}: {}", idx, dir);
            idx += 1;
        }
        line();

        // get input
        println!("Which one you want to added ? (enter nothing to skip) : ");
        let mut str = String::new();
        _ = io::stdin().read_line(&mut str);

        let str = str.trim();

        // matching input & removing
        let val = match str.parse::<usize>() {
            Ok(val) => val,
            Err(_) => break,
        };
        let a = dirs.remove(val);
        added.push(a);
    }

    line();
    println!("added :");
    for r in &added {
        println!("\t{:<4}", r)
    }
    line();
    added
}
