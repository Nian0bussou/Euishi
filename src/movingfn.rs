use crate::{
    counting::GLOBAL_COUNTS,
    utils::{self, errorPrint},
};
use image::image_dimensions;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// # creates the necessary directory :
/// let dwall = Path::new(&dir).join("wall");
/// let dother = Path::new(&dir).join("other");
/// let dsquare = Path::new(&dir).join("square");
/// let dbadquality = Path::new(&dir).join("bad_quality");
/// let dbadqualitylandscape = Path::new(&dbadquality).join("l");
/// let dbadqualitysquare = Path::new(&dbadquality).join("s");
/// let dbadqualityportrait = Path::new(&dbadquality).join("p");
/// let dvideo = Path::new(&dir).join("video");
///
/// let destinations: Vec<&PathBuf> = vec![
///     &dwall,
///     &dother,
///     &dsquare,
///     &dbadquality,
///     &dbadqualitylandscape,
///     &dbadqualitysquare,
///     &dbadqualityportrait,
///     &dvideo,
/// ];
///
/// # then it invoke [`move_file`]
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

/// # classify the file :
/// using width >= 1080 && height >= 1080
/// and aspect ratio
///
/// then invoke [`wrap_rename`]
///
/// # Panics
///
/// Panics if [`wrap_rename`] panics
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
        /* always? width == 1 && height == 1 */
        wrap_rename(file, derrors, "red", "error", source)
    }
}

/// # wrap
/// create the source file & destination file for [`fs::rename`].
///
/// # valid format :
/// "land"
/// "portrait"
/// "square"
/// "video"
///
/// # side effects:
/// increment the following fields of [`GLOBAL_COUNTS`].
/// "landscapes"
/// "portraits"
/// "squares"
/// "video"
///
/// # Panics
///
/// Panics if [`wrap_move`] panics.
fn wrap_rename(file_path: PathBuf, destination: &PathBuf, color: &str, format: &str, source: &str) {
    {
        let mut guard = GLOBAL_COUNTS.lock().unwrap();
        match format {
            "land" => guard.fieldPP("land"),
            "portrait" => guard.fieldPP("port"),
            "square" => guard.fieldPP("squa"),
            "video" => guard.fieldPP("vide"),
            "error" => (),
            _ => panic!("invalid entry match format"),
        };
    }
    let file_name = file_path.file_name().unwrap();
    let new_file = destination.join(file_name);
    wrap_move(file_path, new_file, color, format, source)
}

/// # move the file
/// will increment "succeed" & "failed" fielf of [`GLOBAL_COUNTS`]
///
/// # Panics
///
/// Panics if locking [`GLOBAL_COUNTS`] fails
fn wrap_move(file_path: PathBuf, new_path: PathBuf, color: &str, format: &str, source: &str) {
    let tmp = new_path.parent().unwrap().to_owned();
    let parent_new = tmp.to_str().unwrap();

    // use match because using unwrap hangs the code
    let mut guard = match GLOBAL_COUNTS.lock() {
        Ok(g) => g,
        Err(_) => panic!("couldn't lock guard"),
    };
    guard.fieldPP("proc");
    match fs::rename(file_path, new_path) {
        Ok(_) => {
            guard.fieldPP("succ");
            utils::file_output(source, parent_new, color, format)
        }
        Err(err) => {
            guard.fieldPP("fail");
            utils::errorPrint(err.to_string());
        }
    }
}

/// creates the directories in [dests]
///
/// # Panics
///
/// Panics if cannot lock [`GLOBAL_COUNTS`]
fn make_folders(dests: &Vec<&PathBuf>) {
    for d in dests {
        let mut guard = GLOBAL_COUNTS.lock().unwrap();
        match fs::create_dir(d) {
            Ok(_) => guard.fieldPP("dir"),
            _ => (),
        };
    }
}
