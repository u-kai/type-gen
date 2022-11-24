use std::{
    fs::{self, create_dir_all, read_to_string, File},
    path::{Path, PathBuf},
};

use cli::writer::{all_mkdir, mv_files};
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
    let files = all_file_path(src)
        .into_iter()
        .map(|path| path.to_str().unwrap().to_string())
        .collect();
    all_mkdir(mv_files(files, "examples/jsons", "examples/dist", "rs"))
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
