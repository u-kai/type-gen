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

    fn 受け取ったconvertorに従って受け取ったfile構造体を変換する() {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathStructure {
    root: String,
    path: String,
    extension: Extension,
}

impl PathStructure {
    pub fn new(
        root: impl Into<String>,
        path: impl Into<String>,
        extension: impl Into<Extension>,
    ) -> Self {
        Self {
            root: root.into(),
            path: path.into(),
            extension: extension.into(),
        }
    }
    pub fn to_dist(&self, dist_root: impl Into<String>, dist_extension: Extension) -> Self {
        let dist_root = dist_root.into();
        let dist_path = Extension::repalace(
            &self.path.replace(&self.root, &dist_root),
            &self.extension,
            &dist_extension,
        );
        Self {
            root: dist_root,
            path: dist_path,
            extension: dist_extension,
        }
    }
}
#[test]
fn パスのルートを変更する() {
    let sut = PathStructure::new("./src", "./src/main.rs", "rs");

    let result = sut.to_dist("./dist", Extension::Go);

    assert_eq!(result, PathStructure::new("./dist", "./dist/main.go", "go"));
}

// pub struct DirectoryConvertor {
//     source: RootPath<P>,
//     extension: Extension,
// }
// impl<P> DirectoryConvertor<P>
// where
//     P: AsRef<Path>,
// {
//     pub fn new(source: RootPath<P>, extension: Extension) -> Self {
//         Self { source, extension }
//     }
//     pub fn convert_dirs<'a>(&'a self, dist_root: &'a str) -> impl Iterator<Item = String> + '_ {
//         self.source
//             .childrens
//             .iter()
//             .filter_map(|p| {
//                 if !self.is_this_extension(p) {
//                     return None;
//                 }
//                 Some(
//                     self.replace_src_to_dist(p, dist_root)?
//                         .replace(p.as_ref().file_name()?.to_str()?, ""),
//                 )
//             })
//             .collect::<BTreeSet<_>>()
//             .into_iter()
//     }
//     pub fn convert_files<'a>(
//         &'a self,
//         dist_root: &'a str,
//         target_extension: Extension,
//     ) -> impl Iterator<Item = String> + '_ {
//         self.source.childrens.iter().filter_map(move |p| {
//             if !self.is_this_extension(p) {
//                 return None;
//             }
//             let path = self.replace_src_to_dist(p, dist_root)?;
//             Some(Extension::repalace(
//                 &path,
//                 &self.extension,
//                 &target_extension,
//             ))
//         })
//     }
//     fn replace_src_to_dist<'a>(&self, src: &P, dist_root: &'a str) -> Option<String> {
//         let path = src.as_ref();
//         let path = path
//             .as_os_str()
//             .to_str()?
//             .replace(self.source.root.as_str(), dist_root);
//         Some(path)
//     }
//     fn is_this_extension(&self, p: &P) -> bool {
//         let path = p.as_ref();
//         if let Some(Some(extension)) = path.extension().map(|p| p.to_str()) {
//             extension == self.extension.to_str()
//         } else {
//             false
//         }
//     }
// }
// #[cfg(test)]
// mod directory_convertor_tests {
//     use super::*;
//     use crate::filedatas::extension::Extension;

//     #[test]
//     fn ディレクトリの一覧を別のディレクトリ一覧に変換する() {
//         let source = RootPath {
//             childrens: vec![
//                 "./src/main.rs",
//                 "./src/lib/test.rs",
//                 "./src/lib/child.rs",
//                 "./src/docs/README.md",
//             ],
//             root: "src".to_string(),
//         };

//         let source_extension = Extension::Rs;

//         let sut = DirectoryConvertor::new(source, source_extension);
//         let mut result = sut.convert_dirs("dist");
//         assert_eq!(result.next().unwrap(), "./dist/".to_string());
//         assert_eq!(result.next().unwrap(), "./dist/lib/".to_string());
//     }
//     #[test]
//     fn ファイルパスの一覧を別のファイルパスの一覧に変換する() {
//         let source = RootPath {
//             childrens: vec!["./src/main.rs", "./src/lib/test.rs", "./src/docs/README.md"],
//             root: "src".to_string(),
//         };

//         let source_extension = Extension::Rs;

//         let sut = DirectoryConvertor::new(source, source_extension);
//         let mut result = sut.convert_files("dist", Extension::Go);
//         assert_eq!(result.next().unwrap(), "./dist/main.go".to_string(),);
//         assert_eq!(result.next().unwrap(), "./dist/lib/test.go".to_string(),);
//         assert_eq!(result.next(), None);
//     }
// }
