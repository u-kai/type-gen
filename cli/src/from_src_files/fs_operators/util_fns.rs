use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn all_file_path(root_dir_path: impl AsRef<Path>) -> Vec<PathBuf> {
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
                        let mut files = all_file_path(path);
                        results.append(&mut files);
                        return;
                    }
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

pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir() || path.as_ref().extension().is_none()
}

pub fn mkdir(path: impl AsRef<Path>) {
    if !path.as_ref().exists() {
        fs::create_dir(path.as_ref()).unwrap();
    }
}
#[cfg(test)]
mod test_util_fns {
    use super::{all_file_path, is_dir};
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
