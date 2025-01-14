use crate::{
    counting::pcount,
    getjson::g_json_attrs,
    utils::{self, error_print},
};
use image::image_dimensions;
use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

pub fn move_stuff(dir: String, choose: Option<String>) {
    // making the directories used
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
    // make folders
    for d in &destinations {
        match fs::create_dir(d) {
            Ok(_) => pcount("dir"),
            _ => (),
        };
    }
    match fs::read_dir(&dir) {
        Ok(entries) => {
            for f in entries {
                match f {
                    Ok(f) => {
                        let path = f.path();
                        if path.is_dir() {
                            continue;
                        }
                        move_file(path, &destinations, choose.clone())
                    }
                    Err(err) => error_print(err.to_string()),
                }
            }
        }
        Err(err) => error_print(err.to_string()),
    }
}

fn move_file(file: PathBuf, dests: &Vec<&PathBuf>, choose: Option<String>) {
    // get max width/height via conf.json

    let (min_w, min_h) = match choose {
        Some(path) => {
            let c = g_json_attrs(path);
            (c.minwidth, c.minheight)
        }
        None => (1080, 1080),
    };

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
        wrap_rename(file, dvideo, "yellow", "video");
        return;
    }
    let (width, height) = match image_dimensions(&file) {
        Ok(val) => val,
        Err(_) => (1, 1),
    };
    let aspect_ratio = width as f32 / height as f32;

    if width >= min_w && height >= min_h {
        if aspect_ratio > 1.0 {
            wrap_rename(file, dwall, "red", "land");
        } else if aspect_ratio < 1.0 {
            wrap_rename(file, dother, "green", "portrait")
        } else if aspect_ratio == 1.0 {
            wrap_rename(file, dsquare, "blue", "square")
        }
    } else if width != 1 && height != 1 {
        if aspect_ratio > 1.0 {
            wrap_rename(file, dblandscape, "cyan", "land");
        } else if aspect_ratio < 1.0 {
            wrap_rename(file, dbportrait, "purple", "portrait")
        } else if aspect_ratio == 1.0 {
            wrap_rename(file, dbsquare, "magenta", "square")
        }
    } else {
        wrap_rename(file, derrors, "red", "error")
    }
}

fn wrap_rename(file_path: PathBuf, destination: &PathBuf, color: &str, format: &str) {
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
    let tmp = new_file.parent().unwrap().to_owned();
    let parent_new = tmp.to_str().unwrap();

    pcount("proc");
    match fs::rename(file_path, new_file) {
        Ok(_) => {
            pcount("succ");
            utils::file_output(parent_new, color, format)
        }
        Err(err) => {
            pcount("fail");
            utils::error_print(err.to_string());
        }
    }
}
