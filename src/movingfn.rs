use crate::{
    counting::pcount,
    utils::{self, errorPrint},
};
use image::image_dimensions;
use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

pub fn move_stuff(dir: String) {
    let dwall = Path::new(&dir).join("wall");
    let dother = Path::new(&dir).join("other");
    let dsquare = Path::new(&dir).join("square");
    let dbadquality = Path::new(&dir).join("bad_quality");
    let dbadqualitylandscape = Path::new(&dbadquality).join("l");
    let dbadqualitysquare = Path::new(&dbadquality).join("s");
    let dbadqualityportrait = Path::new(&dbadquality).join("p");
    let dvideo = Path::new(&dir).join("video");
    let derrors = Path::new(&dir).join("errors");
    let destinations: Vec<&PathBuf> = vec![
        &dwall,
        &dother,
        &dsquare,
        &dbadquality,
        &dbadqualitylandscape,
        &dbadqualitysquare,
        &dbadqualityportrait,
        &dvideo,
        &derrors,
    ];
    make_folders(&destinations);

    match fs::read_dir(&dir) {
        Ok(entries) => {
            for f in entries {
                match f {
                    Ok(f) => {
                        let path = f.path();
                        if path.is_dir() {
                            continue;
                        }
                        move_file(path, &destinations, &dir)
                    }
                    Err(err) => errorPrint(err.to_string()),
                }
            }
        }
        Err(err) => errorPrint(err.to_string()),
    }
}

fn move_file(file: PathBuf, dests: &Vec<&PathBuf>, source: &str) {
    let dwall = dests[0];
    let dother = dests[1];
    let dsquare = dests[2];
    let dblandscape = dests[4];
    let dbsquare = dests[5];
    let dbportrait = dests[6];
    let dvideo = dests[7];
    let derrors = dests[8];

    let extension = file.extension().unwrap();
    if extension == "mp4" {
        wrap_rename(file, dvideo, "yellow", "video", source);
        return;
    }
    let (width, height) = match image_dimensions(&file) {
        Ok(val) => val,
        Err(_) => (1, 1),
    };
    let aspect_ratio = width as f32 / height as f32;

    if width >= 1080 && height >= 1080 {
        if aspect_ratio > 1.0 {
            wrap_rename(file, dwall, "red", "land", source);
        } else if aspect_ratio < 1.0 {
            wrap_rename(file, dother, "green", "portrait", source)
        } else if aspect_ratio == 1.0 {
            wrap_rename(file, dsquare, "blue", "square", source)
        }
    } else if width != 1 && height != 1 {
        if aspect_ratio > 1.0 {
            wrap_rename(file, dblandscape, "cyan", "land", source);
        } else if aspect_ratio < 1.0 {
            wrap_rename(file, dbportrait, "purple", "portrait", source)
        } else if aspect_ratio == 1.0 {
            wrap_rename(file, dbsquare, "magenta", "square", source)
        }
    } else {
        wrap_rename(file, derrors, "red", "error", source)
    }
}

fn wrap_rename(file_path: PathBuf, destination: &PathBuf, color: &str, format: &str, source: &str) {
    match format {
        "land" => pcount("land"),
        "portrait" => pcount("port"),
        "square" => pcount("squa"),
        "video" => pcount("vide"),
        "error" => (),
        _ => panic!("invalid entry match format"),
    };

    let file_name = OsString::from(
        file_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace(" ", "-"),
    );

    let new_file = destination.join(file_name);
    wrap_move(file_path, new_file, color, format, source)
}

fn wrap_move(file_path: PathBuf, new_path: PathBuf, color: &str, format: &str, source: &str) {
    let tmp = new_path.parent().unwrap().to_owned();
    let parent_new = tmp.to_str().unwrap();
    pcount("proc");
    match fs::rename(file_path, new_path) {
        Ok(_) => {
            pcount("succ");
            utils::file_output(source, parent_new, color, format)
        }
        Err(err) => {
            pcount("fail");
            utils::errorPrint(err.to_string());
        }
    }
}

fn make_folders(dests: &Vec<&PathBuf>) {
    for d in dests {
        match fs::create_dir(d) {
            Ok(_) => pcount("dir"),
            _ => (),
        };
    }
}
