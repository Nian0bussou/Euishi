#![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]
use crate::counting::get_process;
use std::{
    collections::HashSet,
    env::{args, consts::OS},
    fs,
    path::{Path, PathBuf},
    u8,
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
    let tab = "\t";
    let path = Path::new(&source);
    let padded_dir_str = format!("{:<35}", path.display());
    println!(
        "\t{}{}\t{} <|====|> {}{}",
        color, special, padded_dir_str, dest_path, reset
    )
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

pub fn get_path(hasPath: bool) -> String {
    let path: String;
    let args: Vec<_> = args().collect();
    if hasPath {
        path = args[3].clone();
    } else {
        match OS {
            "windows" => path = "D:/grapper/".to_owned(),
            "linux" => path = "/mnt/d/grapper/".to_owned(),
            _ => panic!("cant get path"),
        }
    }
    if !(Path::new(&path).exists()) {
        panic!("directory not found");
    }
    path
}

/// move_scramble: bool,  // true -> move ; false -> scramble
/// doRemoveTmps: bool,   // true -> call removeTmps ; false -> dont call fn
/// haveCustomPath: bool, // true -> has a path ; false -> yse default path
pub fn g_Choices() -> (bool, bool, bool) {
    //  move_scramble :   true -> move            ;  false -> scramble
    //  doRemoveTmps  :   true -> call removeTmps ;  false -> dont call fn
    //  haveCustomPath:   true -> has a path      ;  false -> yse default path
    let move_scramble: bool;
    let doRemoveTmps: bool;
    let haveCustomPath: bool;

    let ass: Vec<_> = args().collect();
    let lass = ass.len();

    if lass >= 2 {
        move_scramble = if let Ok(n) = ass[1].parse::<u8>() {
            n == 1
        } else {
            panic!("cant get choices ; if len >= 2")
        };
    } else {
        move_scramble = true
    }
    if lass >= 3 {
        doRemoveTmps = if let Ok(n) = ass[2].parse::<u8>() {
            n == 1
        } else {
            panic!("cant get choices ; if len >= 3")
        };
    } else {
        doRemoveTmps = false
    }
    if lass >= 4 {
        haveCustomPath = true;
    } else {
        haveCustomPath = false;
    }
    (self.move_scramble, self.doRemoveTmps, self.haveCustomPath)
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

    if p == 0 && l == 0 && s == 0 && o == 0 && v == 0 {
        return;
    }

    println!(
        "            \n\
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
",
        p, l, o, s, v
    );
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
