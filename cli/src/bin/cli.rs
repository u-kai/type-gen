use std::{
    fs::{self, create_dir_all, read_to_string, File},
    path::{Path, PathBuf},
};

use json::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    src: String,
    dist: String,
}
fn main() {
    let json = read_to_string("config.json").unwrap();
    let json: Config = serde_json::from_str(&json).unwrap();
    let src = json.src;
    let files = all_file_path(src);
    mv_files(files, "rs")
}

fn mv_files(src: Vec<PathBuf>, to_ext: &str) {
    src.into_iter().for_each(|path| {
        let front_of_ext = path
            .to_str()
            .unwrap()
            .replace(path.extension().unwrap().to_str().unwrap(), to_ext);
        create_dir_all(front_of_ext).unwrap();
    })
}
fn all_file_path(root_dir_path: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut all_files = Vec::new();
    let root_dir = fs::read_dir(root_dir_path).unwrap();
    root_dir
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| match entry.file_type() {
            Ok(file_type) => Some((file_type, entry.path())),
            Err(_) => None,
        })
        .for_each(|(file_type, path)| {
            if file_type.is_dir() {
                let mut files = all_file_path(path);
                all_files.append(&mut files);
                return;
            }
            all_files.push(path)
        });
    all_files
}
