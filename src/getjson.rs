use std::{fs::File, io::BufReader};

use serde::Deserialize;
use serde_json::from_reader;

#[derive(Deserialize)]
pub struct Config {
    pub minheight: u32,
    pub minwidth: u32,
    pub paths: Vec<String>,
}

pub fn g_json_attrs(path_json: String) -> Config {
    match g_json_attrs_notwrap(path_json) {
        Ok(c) => c,
        Err(_) => panic!("could not get json attrs"),
    }
}

pub fn g_json_attrs_notwrap(path_json: String) -> std::io::Result<Config> {
    let file = File::open(path_json)?;
    let reader = BufReader::new(file);
    let ps: Config = from_reader(reader)?;

    Ok(ps)
}

#[test]
fn test_json() {
    _ = g_json_attrs_notwrap("conf.json".to_string());
}
