use std::{fs::read_to_string, io, path::Path};

pub struct TypeDefineSrcReader<P: AsRef<Path>> {
    all_src_paths: Vec<P>,
}
impl<P: AsRef<Path>> TypeDefineSrcReader<P> {
    pub fn new(all_src_paths: Vec<P>) -> Self {
        Self { all_src_paths }
    }
    pub fn read_all_srcs(self) -> impl Iterator<Item = SrcFile<P>> {
        self.all_src_paths
            .into_iter()
            .filter_map(|src| Self::src_file(src).ok())
    }
    fn src_file(filepath: P) -> io::Result<SrcFile<P>> {
        let contents = read_to_string(filepath.as_ref())?;
        Ok(SrcFile::new(filepath, contents))
    }
}
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
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod test_src_file {
    use super::{SrcFile, TypeDefineSrcReader};
    #[test]
    // this test context is exist test dir
    fn test_read_all_srcs() {
        let all_src_paths = vec![
            "src/from_src_files/test/parent.txt",
            "src/from_src_files/test/child/child.txt",
            "src/from_src_files/test/child/grand_child/grand_child.txt",
        ];
        let src_reader = TypeDefineSrcReader::new(all_src_paths);
        let mut all_srcs = src_reader.read_all_srcs();
        let src = all_srcs.next().unwrap();
        assert_eq!(src.extract_filename().unwrap(), "parent".to_string());
        assert_eq!(src.content(), "this is parent");
        let src = all_srcs.next().unwrap();
        assert_eq!(src.extract_filename().unwrap(), "child".to_string());
        assert_eq!(src.content(), "this is child");
        let src = all_srcs.next().unwrap();
        assert_eq!(src.extract_filename().unwrap(), "grand_child".to_string());
        assert_eq!(src.content(), "this is grand_child");
    }

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
