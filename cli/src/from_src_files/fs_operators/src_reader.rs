use std::path::Path;

use super::util_fns::is_dir;

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
    fn new(path: P, content: impl Into<String>) -> Self {
        Self {
            path,
            content: content.into(),
        }
    }
    pub fn extract_filename(&self) -> Option<String> {
        let with_extension = self.path.as_ref().file_name()?;
        let extension = format!(".{}", self.path.as_ref().extension()?.to_str()?);
        Some(with_extension.to_string_lossy().replace(&extension, ""))
    }
    pub fn contents(&self) -> &str {
        &self.content
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

#[cfg(test)]
mod test_src_file {
    use super::SrcFile;

    #[test]
    fn test_extract_filename() {
        let dummy_content = "";
        let src = SrcFile::new("src/test.txt", dummy_content);
        assert_eq!(src.extract_filename().unwrap(), "test");
        let src = SrcFile::new("src/.test.txt", dummy_content);
        assert_eq!(src.extract_filename().unwrap(), ".test");
        let src = SrcFile::new("src/test", dummy_content);
        assert_eq!(src.extract_filename(), None);
    }
}
