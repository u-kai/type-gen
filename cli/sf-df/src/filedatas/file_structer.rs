use std::path::Path;

pub struct FileStructer<P>
where
    P: AsRef<Path>,
{
    path: P,
    content: String,
}

impl<P> FileStructer<P>
where
    P: AsRef<Path>,
{
    #[cfg(not(target_os = "windows"))]
    pub const SEPARATOR: &'static str = r#"/"#;
    #[cfg(any(target_os = "windows"))]
    pub const SEPARATOR: &'static str = "\\";

    pub fn new(path: P, content: impl Into<String>) -> Self {
        Self {
            path,
            content: content.into(),
        }
    }
    pub fn name_without_extension(&self) -> Option<&str> {
        let path: &Path = &self.path.as_ref();
        if let Some(Some(path)) = path.file_name().map(|p| p.to_str()) {
            if let Some(extension_index) = path.find('.') {
                return Some(&path[..extension_index]);
            }
            Some(path)
        } else {
            None
        }
    }
}

#[test]
fn pathと拡張子が取り除かれたファイル名を返す() {
    let sut = FileStructer::new("/src/main.rs", "fn main(){}");

    assert_eq!(sut.name_without_extension().unwrap(), "main");
    let sut = FileStructer::new("../src/lib.rs", "fn main(){}");

    assert_eq!(sut.name_without_extension().unwrap(), "lib");
}
