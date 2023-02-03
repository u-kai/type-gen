use std::{fs::read_to_string, path::Path};

use sf_df::fileoperator::create_new_file;
pub struct TestDirectoryOperator {
    paths: Vec<String>,
}
impl TestDirectoryOperator {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }
    pub fn clean_up_before_test(&self, root: &str) {
        std::fs::remove_dir_all(root).unwrap_or_default();
    }
    pub fn prepare_file(&mut self, path: impl Into<String>, content: impl Into<String>) {
        let path = path.into();
        let content = content.into();
        create_new_file(path.clone(), content.clone());
        self.paths.push(path);
    }
    pub fn assert_exist_with_content(
        &mut self,
        path: impl Into<String>,
        content: impl Into<String>,
    ) {
        let path = path.into();
        let content = content.into();
        assert!(Path::new(&path).exists());
        assert_eq!(content, read_to_string(&path).unwrap());
        self.paths.push(path);
    }
    pub fn remove_file(&self, file_name: &str) {
        std::fs::remove_file(file_name).unwrap_or_default();
    }
    pub fn clean_up(self) {
        self.paths
            .into_iter()
            .for_each(|p| std::fs::remove_file(p).unwrap_or_default())
    }
}
