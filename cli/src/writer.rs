use std::path::Path;

pub fn get_dir(path: impl AsRef<Path>) -> String {
    let filename = path.as_ref().file_name().unwrap().to_str().unwrap();
    path.as_ref().to_str().unwrap().replace(filename, "")
}

pub fn split_dirs(path: impl AsRef<Path>) -> Vec<String> {
    let all_dir = get_dir(path);
    let mut result = Vec::new();
    let splited_dirs = all_dir
        .split("/")
        .into_iter()
        .filter(|s| *s != "." && *s != "")
        .collect::<Vec<_>>();
    let mut dir = String::new();
    for i in 0..splited_dirs.len() {
        dir += &format!("{}/", splited_dirs[i]);
        result.push(dir.clone());
    }
    result
}
mod test_file_operations {
    use super::*;
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
