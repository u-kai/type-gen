use std::path::PathBuf;

use super::util_fns::all_file_path;

pub struct SrcPaths<'a> {
    src: &'a str,
    all_path: Vec<PathBuf>,
}
impl<'a> SrcPaths<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            all_path: all_file_path(src),
        }
    }
    #[cfg(test)]
    pub fn for_test(src: &'a str, all_path: Vec<&str>) -> Self {
        Self {
            src,
            all_path: all_path.into_iter().map(|s| s.into()).collect(),
        }
    }
    pub fn src(&self) -> &'a str {
        self.src
    }
    pub fn all_src(&self) -> &Vec<PathBuf> {
        &self.all_path
    }
}
