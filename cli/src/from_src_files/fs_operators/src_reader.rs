pub struct SrcFile<P>
where
    P: AsRef<Path>,
{
    path: P,
    content: String,
}

impl<P> SrcFile<P>
where
    P: AsRef<Path>,
{
    fn new(path: P, content: String) -> Self {
        Self { path, content }
    }
    pub fn extract_filename(&self) -> String {
        String::new()
    }
    pub fn contents(&self) -> &str {
        ""
    }
}
pub struct TypeDefineSrcReader<P: AsRef<Path>> {
    all_src_files: Vec<P>,
}
impl<P: AsRef<Path>> TypeDefineSrcReader<P> {
    pub fn new(all_src_files: Vec<P>) -> Self {
        Self { all_src_files }
    }
    //pub fn all_src_filename_and_contents(&self) -> Vec<(String, String)> {
    //self.all_src_files
    //.iter()
    //.map(|src| self.filename_and_contents(src))
    //.collect()
    //}
    //fn filename_and_contents<P: AsRef<Path>>(&self, filepath: P) -> (String, String) {
    //let filepath = filepath.as_ref();
    //let contents = read_to_string(filepath).unwrap();
    //(extract_filename(filepath), contents)
    //}
}

use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf},
};
