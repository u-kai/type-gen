use std::{
    fs::{self, create_dir_all, read_to_string, File},
    path::{Path, PathBuf},
};

use cli::{
    type_configuration::ConfigJson,
    writer::{all_mkdir, mv_files},
};
use json::json::Json;
use langs::rust::builder::RustTypeDefainGeneratorBuilder;

fn main() {
    let json = read_to_string("config.json").unwrap();

    let json: ConfigJson = serde_json::from_str(&json).unwrap();
    println!("{:#?}", json);
    let builder = RustTypeDefainGeneratorBuilder::new();
    let definer = json.to_definer(builder);
    let type_structure = Json::from(r#"{"key":"value"}"#).into_type_structures("Test");
    let statements = definer
        .generate(type_structure)
        .into_iter()
        .reduce(|acc, cur| format!("{}{}\n", acc, cur))
        .unwrap();
    println!("{}", statements);

    //let src = json.src;
    //let files = all_file_path(src)
    //.into_iter()
    //.map(|path| path.to_str().unwrap().to_string())
    //.collect();
    //all_mkdir(mv_files(files, "examples/jsons", "examples/dist", "rs"))
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
