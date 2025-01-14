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

struct Directories {
    pub wall: PathBuf,
    pub other: PathBuf,
    pub square: PathBuf,
    pub bad_quality: PathBuf,
    pub bad_quality_landscape: PathBuf,
    pub bad_quality_square: PathBuf,
    pub bad_quality_portrait: PathBuf,
    pub video: PathBuf,
    pub errors: PathBuf,
}

impl Directories {
    fn new(base_dir: &str) -> Self {
        let bad_quality = Path::new(base_dir).join("bad_quality");
        let dirs = Directories {
            wall: Path::new(base_dir).join("wall"),
            other: Path::new(base_dir).join("other"),
            square: Path::new(base_dir).join("square"),
            bad_quality: bad_quality.clone(),
            bad_quality_landscape: bad_quality.join("l"),
            bad_quality_square: bad_quality.join("s"),
            bad_quality_portrait: bad_quality.join("p"),
            video: Path::new(base_dir).join("video"),
            errors: Path::new(base_dir).join("errors"),
        };
        dirs.create_all();
        dirs
    }

    fn create_all(&self) {
        let paths = [
            &self.wall,
            &self.other,
            &self.square,
            &self.bad_quality,
            &self.bad_quality_landscape,
            &self.bad_quality_square,
            &self.bad_quality_portrait,
            &self.video,
            &self.errors,
        ];
        for path in paths.iter() {
            match fs::create_dir(path) {
                Ok(_) => pcount("dir"),
                _ => (),
            };
        }
    }
}

pub fn move_stuff(dir: String) {
    let directories = Directories::new(&dir);

    match fs::read_dir(&dir) {
        Ok(entries) => {
            for f in entries {
                match f {
                    Ok(f) => {
                        let path = f.path();
                        if path.is_dir() {
                            continue;
                        }
                        move_file(path, &directories)
                    }
                    Err(err) => error_print(err.to_string()),
                }
            }
        }
        Err(err) => error_print(err.to_string()),
    }
}

fn move_file(file: PathBuf, dests: &Directories) {
    let (min_w, min_h) = (1080, 1080);

    let dwall = &dests.wall;
    let dother = &dests.other;
    let dsquare = &dests.square;
    let dblandscape = &dests.bad_quality_landscape;
    let dbsquare = &dests.bad_quality_square;
    let dbportrait = &dests.bad_quality_portrait;
    let dvideo = &dests.video;
    let derrors = &dests.errors;

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
