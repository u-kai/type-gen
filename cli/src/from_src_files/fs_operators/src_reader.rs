//pub struct TypeDefineSrcReader {
//all_src_files: Vec<PathBuf>,//}//impl TypeDefineSrcReader {
//pub fn new(src: &str) -> Self {
//Self {
//all_src_files: all_file_path(src),
//}
//}
//fn all_src_filepaths(self) -> Vec<PathBuf> {
//self.all_src_files
//}
//fn all_src_filename_and_contents(&self) -> Vec<(String, String)> {
//self.all_src_files
//.iter()
//.map(|src| self.filename_and_contents(src))
//.collect()
//}
//fn filename_and_contents<P: AsRef<Path>>(&self, filepath: P) -> (String, String) {
//let filepath = filepath.as_ref();
//let contents = read_to_string(filepath).unwrap();
//(extract_filename(filepath), contents)
//}//}

use std::{
    fs,
    path::{Path, PathBuf},
};

fn all_file_path(root_dir_path: impl AsRef<Path>) -> Vec<PathBuf> {
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

#[cfg(test)]
mod test_src_reader {
    use super::all_file_path;

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
