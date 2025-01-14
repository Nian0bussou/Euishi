use crate::counting::g_count;
use crate::outfile::make_output;
use std::collections::HashSet;
use std::env::consts::OS;
use std::fs;
use std::panic;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

pub fn file_output(dest_path: &str, color_name: &str, dest_type: &str) {
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
        _ => panic!("invalid color option ; got {}", color_name),
    };
    let special = match dest_type {
        "land" => "━",
        "square" => "■",
        "portrait" => "┃",
        "video" => "▶",
        "error" => "❌",
        _ => panic!("invalid speci option ; got {}", dest_type),
    };
    println!("\t{color}{special}\t{dest_path}{reset}")
}

pub fn error_print(err: String) {
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
            _ => {
                eprintln!("cant get path");
                exit(1)
            }
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
                                                                  // 🏳️<200d>⚧️
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
    let (p, l, s, o, v) = g_count();

    if p == 0 && l == 0 && s == 0 && o == 0 && v == 0 {
        return;
    }

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

    println!("{}", msg);

    if let Err(_) = make_output(msg) {
        eprintln!("couldn't make output file")
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
