use std::{fs::read_to_string, io, path::PathBuf};

use super::src_paths::SrcPaths;

pub struct TypeDefineSrcReader<'a> {
    inner: &'a SrcPaths<'a>,
}
impl<'a> TypeDefineSrcReader<'a> {
    pub fn new(src_paths: &'a SrcPaths) -> Self {
        Self { inner: src_paths }
    }
    pub fn read_all_srcs(&self) -> impl Iterator<Item = SrcFile> + 'a {
        self.inner
            .all_src()
            .iter()
            .filter_map(|src| Self::src_file(src).ok())
    }
    fn src_file(filepath: &PathBuf) -> io::Result<SrcFile> {
        let contents = read_to_string(filepath)?;
        Ok(SrcFile::from_file(filepath, contents))
    }
}
pub struct SrcFile {
    extracted_filename: Option<String>,
    content: String,
}

impl SrcFile {
    pub fn extracted_filename(&self) -> Option<&str> {
        self.extracted_filename.as_ref().map(|s| s.as_str())
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    fn from_file(path: &PathBuf, content: impl Into<String>) -> Self {
        let extracted_filename = Self::extract_filename(path);
        Self {
            extracted_filename,
            content: content.into(),
        }
    }
    #[cfg(test)]
    fn from_path_str(path: &str, content: impl Into<String>) -> Self {
        let path: PathBuf = path.into();
        let extracted_filename = Self::extract_filename(&path);
        Self {
            extracted_filename,
            content: content.into(),
        }
    }
    fn extract_filename(path: &PathBuf) -> Option<String> {
        let with_extension = path.file_name()?;
        let extension = format!(".{}", path.extension()?.to_str()?);
        Some(with_extension.to_string_lossy().replace(&extension, ""))
    }
}

#[cfg(test)]
mod test_src_file {
    use crate::from_src_files::fs_operators::src_paths::SrcPaths;

    use super::{SrcFile, TypeDefineSrcReader};
    #[test]
    // this test context is exist test dir
    fn test_read_all_srcs() {
        let all_src_paths = vec![
            "src/from_src_files/test/parent.txt",
            "src/from_src_files/test/child/child.txt",
            "src/from_src_files/test/child/grand_child/grand_child.txt",
        ];
        let src_paths = SrcPaths::for_test("src", all_src_paths);
        let src_reader = TypeDefineSrcReader::new(&src_paths);
        let mut all_srcs = src_reader.read_all_srcs();
        let src = all_srcs.next().unwrap();
        assert_eq!(src.extracted_filename().unwrap(), "parent");
        assert_eq!(src.content(), "this is parent");
        let src = all_srcs.next().unwrap();
        assert_eq!(src.extracted_filename().unwrap(), "child");
        assert_eq!(src.content(), "this is child");
        let src = all_srcs.next().unwrap();
        assert_eq!(src.extracted_filename().unwrap(), "grand_child");
        assert_eq!(src.content(), "this is grand_child");
    }

    #[test]
    fn test_extracted_filename() {
        let dummy_content = "";
        let src = SrcFile::from_path_str("src/test.txt", dummy_content);
        assert_eq!(src.extracted_filename().unwrap(), "test");
        let src = SrcFile::from_path_str("src/.test.txt", dummy_content);
        assert_eq!(src.extracted_filename().unwrap(), ".test");
        let src = SrcFile::from_path_str("src/test", dummy_content);
        assert_eq!(src.extracted_filename(), None);
    }
}
