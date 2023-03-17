use std::{
    fs::{self, read_to_string, File, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::{
    extension::Extension,
    fileconvertor::{FileStructer, PathStructure},
};
impl FileStructer {
    pub fn add_new_line_to_file(&self) {
        add_to_file(self.path().path_str(), format!("\n{}", self.content()));
    }
    pub fn add_to_file(&self) {
        add_to_file(self.path().path_str(), self.content())
    }
    pub fn new_file(&self) {
        create_new_file(self.path().path_str(), self.content())
    }
}

pub fn file_structures_to_files(v: &Vec<FileStructer>) {
    v.iter().for_each(|f| f.new_file());
}
#[cfg(not(target_os = "windows"))]
pub const SEPARATOR: &'static str = r#"/"#;
#[cfg(any(target_os = "windows", feature = "test_win"))]
pub const SEPARATOR: &'static str = "\\";
pub fn all_file_path(root_dir_path: impl AsRef<Path>) -> Vec<PathBuf> {
    match fs::read_dir(root_dir_path.as_ref()) {
        Ok(root_dir) => root_dir
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| match entry.file_type() {
                Ok(file_type) => Some((file_type, entry.path())),
                Err(_) => None,
            })
            .fold(Vec::new(), |mut acc, (file_type, path)| {
                if file_type.is_dir() {
                    let mut files = all_file_path(path);
                    acc.append(&mut files);
                    return acc;
                }
                acc.push(path);
                acc
            }),
        Err(e) => {
            println!("{}", e.to_string());
            panic!("not found path = {:?}", root_dir_path.as_ref())
        }
    }
}
pub fn add_to_file(path: impl AsRef<Path>, content: impl Into<String>) {
    let path = path.as_ref();
    let content = content.into();
    let mut file = if path.exists() {
        OpenOptions::new()
            .append(true)
            .open(path)
            .expect(&format!("path {:?} can not added , ", path,))
    } else {
        File::create(path).expect(&format!("path can not write {:?}", path,))
    };
    file.write_all(content.as_bytes()).expect(&format!(
        "path {:?} can not write,\ncontent {}",
        path, content
    ));
}
pub fn create_new_file(path: impl AsRef<Path>, content: impl Into<String>) {
    let content: String = content.into();
    if path.as_ref().exists() {
        let mut writer = BufWriter::new(File::create(path).unwrap());
        writer.write_all(content.as_bytes()).unwrap();
        return;
    }
    prepare_parents(path.as_ref());
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writer.write_all(content.as_bytes()).unwrap();
}
fn prepare_parents(path: impl AsRef<Path>) {
    if path.as_ref().exists() {
        return;
    }
    let filename = path
        .as_ref()
        .file_name()
        .map(|f| f.to_str())
        .unwrap_or_default()
        .unwrap_or_default();
    let dirs = path
        .as_ref()
        .to_str()
        .unwrap_or_default()
        .replacen(filename, "", 1);
    mkdir_rec(dirs).unwrap();
}
pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir() || path.as_ref().extension().is_none()
}

pub fn mkdir_rec(path: impl AsRef<Path>) -> Result<(), String> {
    let Some(path) = split_dirs(path.as_ref()) else {
        return  Err(format!("not splited dir {:#?}",path.as_ref().to_str()))
    };
    Ok(path.for_each(|dir| mkdir(dir)))
}
fn mkdir(path: impl AsRef<Path>) {
    if !path.as_ref().exists() {
        fs::create_dir(path.as_ref()).expect(&format!("{:#?} is not create dir", path.as_ref()));
    }
}
fn split_dirs(path: impl AsRef<Path>) -> Option<impl Iterator<Item = String>> {
    let all_dir = extract_dir(path)?;
    let mut dir = String::new();
    Some(
        all_dir
            .split(SEPARATOR)
            .into_iter()
            .filter(|s| *s != "." && *s != "")
            .fold(Vec::new().clone(), |mut acc, s| {
                dir += &format!("{}{}", s, SEPARATOR);
                acc.push(dir.clone());
                acc
            })
            .into_iter(),
    )
}
fn extract_dir<P: AsRef<Path>>(path: P) -> Option<String> {
    if is_dir(path.as_ref()) {
        return path.as_ref().to_str().map(|s| s.to_string());
    }
    let filename = path.as_ref().file_name()?.to_str()?;
    path.as_ref().to_str().map(|s| s.replace(filename, ""))
}
pub fn all_file_structure(root: &str, extension: impl Into<Extension>) -> Vec<FileStructer> {
    let extension: Extension = extension.into();
    if extension.is_this_extension(root) {
        return vec![FileStructer::from_path(root)];
    }
    all_file_path(root)
        .iter()
        .filter(move |p| extension.is_this_extension(p))
        .map(|p| {
            let path = p.to_str().unwrap_or_default();
            FileStructer::new(
                read_to_string(p).unwrap(),
                PathStructure::new(path, extension),
            )
        })
        .collect()
}
