use std::{
    fs,
    path::{Path, PathBuf},
};

#[cfg(not(target_os = "windows"))]
pub const SEPARATOR: &'static str = r#"/"#;
#[cfg(any(target_os = "windows", feature = "test_win"))]
pub const SEPARATOR: &'static str = r#"\\"#;

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
            panic!()
        }
    }
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
pub fn mkdir(path: impl AsRef<Path>) {
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
pub fn extract_dir<P: AsRef<Path>>(path: P) -> Option<String> {
    if is_dir(path.as_ref()) {
        return path.as_ref().to_str().map(|s| s.to_string());
    }
    let filename = path.as_ref().file_name()?.to_str()?;
    path.as_ref().to_str().map(|s| s.replace(filename, ""))
}
#[cfg(test)]
fn all_path(root_dir_path: impl AsRef<Path>) -> Vec<PathBuf> {
    match fs::read_dir(root_dir_path.as_ref()) {
        Ok(root_dir) => {
            let mut results = Vec::new();
            root_dir
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| match entry.file_type() {
                    Ok(file_type) => Some((file_type, entry.path())),
                    Err(_) => None,
                })
                .for_each(|(file_type, path)| {
                    if file_type.is_dir() {
                        let mut files = all_path(path.clone());
                        results.append(&mut files);
                    }
                    println!("path = {:?}", path);
                    results.push(path)
                });

            results
        }
        Err(e) => {
            println!("{}", e.to_string());
            panic!()
        }
    }
}
#[cfg(any(target_os = "windows", feature = "test_win", test))]
mod test_util_fns_win {

    use crate::from_src_files::fs_operators::util_fns::{all_path, extract_dir, split_dirs};

    use super::{all_file_path, is_dir, mkdir_rec};
    #[test]
    fn test_mkdir_rec() {
        let path = "src/from_src_files/mkdir/mkdir_rec/mkdir_rec_child";
        mkdir_rec(path).unwrap();
        let results = all_path("src");
        assert!(results.contains(&"src/from_src_files/mkdir/".into()));
        assert!(results.contains(&"src/from_src_files/mkdir/mkdir_rec/".into()));
        assert!(results.contains(&"src/from_src_files/mkdir/mkdir_rec/mkdir_rec_child".into()));
        // clean up not use watch test
        // if you use above code under cargo watch test context
        // cause infinite loop
        // std::fs::remove_dir_all("src/from_src_files/mkdir").unwrap()
    }
    #[test]
    fn test_split_dirs() {
        let path = "./src/example/child/test.txt";
        let mut splited = split_dirs(path).unwrap();
        assert_eq!(splited.next().unwrap(), "src/");
        assert_eq!(splited.next().unwrap(), "src/example/");
        assert_eq!(splited.next().unwrap(), "src/example/child/");
        assert_eq!(splited.next(), None);
        let path = "./src/example/child/";
        let mut splited = split_dirs(path).unwrap();
        assert_eq!(splited.next().unwrap(), "src/");
        assert_eq!(splited.next().unwrap(), "src/example/");
        assert_eq!(splited.next().unwrap(), "src/example/child/");
        assert_eq!(splited.next(), None);
    }
    #[test]
    fn test_extract_dir() {
        let path = "src/dist/test.txt";
        assert_eq!(extract_dir(&path).unwrap(), "src/dist/".to_string());
        let path = "src/dist/";
        assert_eq!(extract_dir(&path).unwrap(), "src/dist/".to_string());
    }
    #[test]
    fn test_is_dir() {
        let dir = "src/dist/";
        assert!(is_dir(dir));
        let file = "src/dist/test.txt";
        assert!(!is_dir(file));
    }

    #[test]
    fn test_get_all_file_path() {
        // this test context is exist test directory
        let tobe = vec![
            "./src/from_src_files/test/parent.txt".to_string(),
            "./src/from_src_files/test/child/child.txt".to_string(),
            "./src/from_src_files/test/child/grand_child/grand_child.txt".to_string(),
        ];
        assert_eq!(
            all_file_path("./src/from_src_files/test")
                .into_iter()
                .map(|p| p.to_str().unwrap().to_string())
                .collect::<Vec<_>>(),
            tobe
        );
    }
}

#[cfg(all(not(target_os = "windows"), test))]
mod test_util_fns {

    use crate::from_src_files::fs_operators::util_fns::{all_path, extract_dir, split_dirs};

    use super::{all_file_path, is_dir, mkdir_rec};
    #[test]
    fn test_mkdir_rec() {
        let path = "src/from_src_files/mkdir/mkdir_rec/mkdir_rec_child";
        mkdir_rec(path).unwrap();
        let results = all_path("src");
        assert!(results.contains(&"src/from_src_files/mkdir/".into()));
        assert!(results.contains(&"src/from_src_files/mkdir/mkdir_rec/".into()));
        assert!(results.contains(&"src/from_src_files/mkdir/mkdir_rec/mkdir_rec_child".into()));
        // clean up not use watch test
        // if you use above code under cargo watch test context
        // cause infinite loop
        // std::fs::remove_dir_all("src/from_src_files/mkdir").unwrap()
    }
    #[test]
    fn test_split_dirs() {
        let path = "./src/example/child/test.txt";
        let mut splited = split_dirs(path).unwrap();
        assert_eq!(splited.next().unwrap(), "src/");
        assert_eq!(splited.next().unwrap(), "src/example/");
        assert_eq!(splited.next().unwrap(), "src/example/child/");
        assert_eq!(splited.next(), None);
        let path = "./src/example/child/";
        let mut splited = split_dirs(path).unwrap();
        assert_eq!(splited.next().unwrap(), "src/");
        assert_eq!(splited.next().unwrap(), "src/example/");
        assert_eq!(splited.next().unwrap(), "src/example/child/");
        assert_eq!(splited.next(), None);
    }
    #[test]
    fn test_extract_dir() {
        let path = "src/dist/test.txt";
        assert_eq!(extract_dir(&path).unwrap(), "src/dist/".to_string());
        let path = "src/dist/";
        assert_eq!(extract_dir(&path).unwrap(), "src/dist/".to_string());
    }
    #[test]
    fn test_is_dir() {
        let dir = "src/dist/";
        assert!(is_dir(dir));
        let file = "src/dist/test.txt";
        assert!(!is_dir(file));
    }

    #[test]
    fn test_get_all_file_path() {
        // this test context is exist test directory
        let tobe = vec![
            "./src/from_src_files/test/parent.txt".to_string(),
            "./src/from_src_files/test/child/child.txt".to_string(),
            "./src/from_src_files/test/child/grand_child/grand_child.txt".to_string(),
        ];
        assert_eq!(
            all_file_path("./src/from_src_files/test")
                .into_iter()
                .map(|p| p.to_str().unwrap().to_string())
                .collect::<Vec<_>>(),
            tobe
        );
    }
}
