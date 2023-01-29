use std::{
    fs::{self, read_to_string, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::{
    extension::Extension,
    fileconvertor::{FileStructer, PathStructure},
};

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

pub fn create_file(path: impl AsRef<Path>, content: impl Into<String>) {
    let content: String = content.into();
    if path.as_ref().exists() {
        let mut writer = BufWriter::new(File::create(path).unwrap());
        writer.write_all(content.as_bytes()).unwrap();
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
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writer.write_all(content.as_bytes()).unwrap();
}
fn is_dir<P: AsRef<Path>>(path: P) -> bool {
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
    all_file_path(root)
        .iter()
        .filter(move |p| extension.is_this_extension(p))
        .map(|p| {
            let path = p.to_str().unwrap_or_default();
            FileStructer::new(
                read_to_string(p).unwrap(),
                PathStructure::new(root, path, extension),
            )
        })
        .collect()
}
#[cfg(test)]
mod test_util_fns_win {
    use std::{fs::read_to_string, path::Path};

    use crate::{
        fileconvertor::{FileStructer, PathStructure},
        fileoperator::{all_file_structure, create_file},
    };

    use super::{all_file_path, mkdir_rec};
    #[test]
    fn for_testディレクトリ内の全てのファイルから指定した拡張子だけfilestructureとして生成する() {
        // this test context is exist test directory
        let tobe = vec![
            FileStructer::new(
                read_to_string("./for-test/rust.rs").unwrap(),
                PathStructure::new("./for-test", "./for-test/rust.rs", "rs"),
            ),
            FileStructer::new(
                read_to_string("./for-test/child/rust_child.rs").unwrap(),
                PathStructure::new("./for-test", "./for-test/child/rust_child.rs", "rs"),
            ),
        ];
        assert_eq!(all_file_structure("./for-test", "rs"), tobe);
    }
    #[test]
    fn for_testディレクトリ内の全てのファイルのパスを取得する() {
        // this test context is exist test directory
        let tobe = vec![
            "./for-test/parent.txt".to_string(),
            "./for-test/rust.rs".to_string(),
            "./for-test/child/child.txt".to_string(),
            "./for-test/child/grand_child/grand_child.txt".to_string(),
            "./for-test/child/rust_child.rs".to_string(),
        ];
        assert_eq!(
            all_file_path("./for-test")
                .into_iter()
                .map(|p| p.to_str().unwrap().to_string())
                .collect::<Vec<_>>(),
            tobe
        );
    }
    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn 存在しない指定されたディレクトリを再起的に生成する() {
        let path = "./mkdir/mkdir_rec/mkdir_rec_child";
        let _sut = mkdir_rec(path).unwrap();

        assert!(Path::new("mkdir/").exists());
        assert!(Path::new("mkdir/mkdir_rec/").exists(),);
        assert!(Path::new("mkdir/mkdir_rec/mkdir_rec_child").exists(),);

        //crean up
        std::fs::remove_dir_all("mkdir").unwrap()
    }
    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn 指定されたファイルパスを存在しないディレクトリも含めて作成する() {
        let new_path = "not-exist/non-exist/new-file.txt";
        let content = "test hello world";

        create_file(new_path, content);

        assert!(Path::new("not-exist").exists());
        assert!(Path::new("not-exist/non-exist").exists());
        assert!(Path::new("not-exist/non-exist/new-file.txt").exists());
        assert_eq!(
            read_to_string("not-exist/non-exist/new-file.txt").unwrap(),
            content
        );

        //crean up
        std::fs::remove_dir_all("not-exist").unwrap()
    }
}
