use std::path::Path;

use crate::filedatas::extension::Extension;

pub trait FileStructerConvertor {
    fn convert(&self, file: &FileStructer, extension: Extension) -> FileStructer;
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileStructer {
    extension: Extension,
    content: String,
    path: String,
}
impl FileStructer {
    pub fn new(
        extension: impl Into<Extension>,
        content: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            extension: extension.into(),
            content: content.into(),
            path: path.into(),
        }
    }
    pub fn name_without_extension(&self) -> &str {
        let path: &Path = self.path.as_ref();
        if let Some(Some(filename)) = path.file_name().map(|filename| filename.to_str()) {
            if let Some(index) = filename.find(".") {
                return &filename[..index];
            }
        }
        &self.path
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn to_dist(&self, extension: impl Into<Extension>, content: impl Into<String>) -> Self {
        let extension = extension.into();
        Self::new(extension, content, self.repalace_extension_path(extension))
    }
    pub fn from_source<F>(&self, extension: impl Into<Extension>, f: F) -> Self
    where
        F: Fn(&str) -> String,
    {
        let extension = extension.into();
        Self::new(
            extension,
            f(&self.content),
            self.repalace_extension_path(extension),
        )
    }
    fn repalace_extension_path(&self, extension: impl Into<Extension>) -> String {
        self.path.replace(
            &format!(".{}", self.extension.to_str()),
            &format!(".{}", extension.into().to_str()),
        )
    }
}
#[cfg(test)]
mod file_structer_tests {
    use super::*;
    #[test]
    fn pathと拡張子が取り除かれたファイル名を返す() {
        let sut = FileStructer::new("rs", "fn main(){}", "src/main.rs");

        assert_eq!(sut.name_without_extension(), "main");
    }
}

pub struct FileConvetor {
    source: Vec<FileStructer>,
}
impl FileConvetor {
    pub fn new(source: Vec<FileStructer>) -> Self {
        Self { source }
    }
    pub fn convert(
        &self,
        extension: Extension,
        convertor: impl FileStructerConvertor,
    ) -> Vec<FileStructer> {
        self.source
            .iter()
            .map(|file| convertor.convert(file, extension))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn 受け取ったcloserに従って受け取ったfile構造体を変換する() {
        let source = vec![
            FileStructer::new("go", "func main(){}", "src/main.go"),
            FileStructer::new("go", "func main(){}", "src/lib/lib.go"),
            FileStructer::new("go", "func main(){}", "src/bin/bin.go"),
        ];
        let sut = FileConvetor::new(source);
        struct FakeConvertor {}
        impl FileStructerConvertor for FakeConvertor {
            fn convert(&self, f: &FileStructer, e: Extension) -> FileStructer {
                f.from_source(e, |s| s.replace("func", "fn"))
            }
        }
        let convertor = FakeConvertor {};
        let result = sut.convert(Extension::Rs, convertor);
        assert_eq!(
            result,
            vec![
                FileStructer::new("rs", "fn main(){}", "src/main.rs"),
                FileStructer::new("rs", "fn main(){}", "src/lib/lib.rs"),
                FileStructer::new("rs", "fn main(){}", "src/bin/bin.rs"),
            ]
        );
    }
}
