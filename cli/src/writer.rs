use std::path::Path;
pub fn mv_files(
    dirs: Vec<impl AsRef<Path>>,
    src: &str,
    dist: &str,
    to_extension: &str,
) -> Vec<String> {
    dirs.into_iter()
        .map(|dir| {
            let dir = dir.as_ref();
            let extension = dir.extension().unwrap().to_str().unwrap();
            let original_filename = dir.file_name().unwrap().to_str().unwrap();
            let new_filename = original_filename.replace(extension, to_extension);
            format!("{}{}", get_dir(dir).replace(src, dist), new_filename)
        })
        .collect()
}

pub fn get_dir(path: impl AsRef<Path>) -> String {
    if path.as_ref().is_dir() {
        return path.as_ref().to_str().unwrap().to_string();
    }
    let filename = path.as_ref().file_name().unwrap().to_str().unwrap();
    path.as_ref().to_str().unwrap().replace(filename, "")
}

pub fn split_dirs(path: impl AsRef<Path>) -> Vec<String> {
    let all_dir = get_dir(path);
    let mut dir = String::new();
    all_dir
        .split("/")
        .into_iter()
        .filter(|s| *s != "." && *s != "")
        .fold(Vec::new(), |mut acc, s| {
            dir += &format!("{}/", s);
            acc.push(dir.clone());
            acc
        })
}
mod test_file_operations {
    use super::*;
    #[test]
    fn test_mv_files() {
        let paths = vec![
            "./src/test.txt",
            "./src/dir1/test.txt",
            "./src/dir2/test.txt",
        ];
        assert_eq!(
            mv_files(paths, "src", "dist", "rs"),
            vec![
                "./dist/test.rs",
                "./dist/dir1/test.rs",
                "./dist/dir2/test.rs",
            ]
        );
    }
    #[test]
    fn test_get_dir() {
        let path = "./dir1/test.txt";
        assert_eq!(get_dir(path), "./dir1/".to_string());
    }
    #[test]
    fn test_split_dirs() {
        let path = "./src/example/child/test.txt";
        assert_eq!(
            split_dirs(path),
            vec![
                "src/".to_string(),
                "src/example/".to_string(),
                "src/example/child/".to_string(),
            ]
        );
    }
}
