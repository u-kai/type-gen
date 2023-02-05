use std::{collections::BTreeSet, path::Path};

use npc::fns::to_snake;

use crate::extension::Extension;

pub trait FileStructerConvertor {
    fn convert(
        &self,
        src_root: &str,
        file: &FileStructer,
        extension: impl Into<Extension>,
    ) -> FileStructer;
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileStructer {
    content: String,
    path: PathStructure,
}
impl FileStructer {
    pub fn new(content: impl Into<String>, path: PathStructure) -> Self {
        Self {
            content: content.into(),
            path: path,
        }
    }
    pub fn to_snake_path(self) -> Self {
        Self::new(self.content, self.path.to_snake_path())
    }
    pub fn name_without_extension(&self) -> &str {
        self.path.name_without_extension()
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn path(&self) -> &PathStructure {
        &self.path
    }
    pub fn to_dist(
        &self,
        src_root: &str,
        dist_root: &str,
        dist_extension: impl Into<Extension>,
        content: impl Into<String>,
    ) -> Self {
        let dist = self.path.to_dist(src_root, dist_root, dist_extension);
        Self::new(content, dist)
    }
}
impl FileStructer {
    pub fn dir_set(v: &Vec<Self>, this_root: &str) -> BTreeSet<String> {
        v.iter()
            .map(|f| f.path.all_child_dirs(this_root))
            .flat_map(|v| v)
            .collect::<BTreeSet<_>>()
    }
}
#[cfg(test)]
mod file_structer_tests {
    use super::*;
    #[test]
    fn pathと拡張子が取り除かれたファイル名を返す() {
        let sut = FileStructer::new("fn main(){}", PathStructure::new("src/main.rs", "rs"));

        assert_eq!(sut.name_without_extension(), "main");
    }
    // #[test]
    // fn file_structuresの配列からそのfile_structureが格納されている全てのディレクトリを返す() {
    //     let source = vec![
    //         FileStructer::new("dummy", PathStructure::new("./tests/rusts/test.rs", "rs")),
    //         FileStructer::new(
    //             "dummy",
    //             PathStructure::new("./tests/rusts/nests/test-child.rs", "rs"),
    //         ),
    //         FileStructer::new(
    //             "dummy",
    //             PathStructure::new("./tests/rusts/nests/child/array.rs", "rs"),
    //         ),
    //         FileStructer::new(
    //             "dummy",
    //             PathStructure::new("./tests/rusts/nests/child/rs-placeholder.rs", "rs"),
    //         ),
    //     ];

    //     let result = FileStructer::dir_set(&source, "tests/rusts")
    //         .into_iter()
    //         .collect::<Vec<_>>();

    //     assert_eq!(
    //         result,
    //         vec![
    //             "tests/rusts/",
    //             "tests/rusts/nests/",
    //             "tests/rusts/nests/child/",
    //         ]
    //     );
    // }
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
        src_root: &str,
        extension: Extension,
        convertor: impl FileStructerConvertor,
    ) -> Vec<FileStructer> {
        self.source
            .iter()
            .map(|file| convertor.convert(src_root, file, extension))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathStructure {
    path: String,
    extension: Extension,
}

impl PathStructure {
    #[cfg(not(target_os = "windows"))]
    pub const SEPARATOR: &'static str = "/";
    #[cfg(any(target_os = "windows", feature = "test_win"))]
    pub const SEPARATOR: &'static str = "\\";

    pub fn new(path: impl Into<String>, extension: impl Into<Extension>) -> Self {
        Self {
            path: path.into(),
            extension: extension.into(),
        }
    }
    pub fn parent_str(&self) -> String {
        let path: &Path = self.path.as_ref();
        if let Some(Some(filename)) = path.file_name().map(|f| f.to_str()) {
            let mut result = self.path.replace(filename, "");
            println!("{}", &result[result.len() - 2..]);
            if &result[result.len() - 2..] == "//" {
                result.pop();
            }
            result
        } else {
            self.path.clone()
        }
    }
    pub fn path_str(&self) -> &str {
        &self.path
    }
    pub fn all_child_dirs(&self, this_root: &str) -> Vec<String> {
        let all_child_dirs = self.extract_dir();
        let mut dir = String::new();
        all_child_dirs
            .split(Self::SEPARATOR)
            .into_iter()
            .filter(|s| *s != "." && *s != "")
            .fold(Vec::new(), |mut acc, s| {
                dir += &format!("{}{}", s, Self::SEPARATOR);
                acc.push(dir.clone());
                acc
            })
            .into_iter()
            .filter(|path| path.len() > this_root.len())
            .collect()
    }
    fn extract_dir(&self) -> String {
        let path: &Path = self.path.as_ref();
        if let Some(Some(filename)) = path.file_name().map(|f| f.to_str()) {
            self.path.replace(filename, "")
        } else {
            self.path.clone()
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
    pub fn to_snake_path(self) -> Self {
        let new_name = to_snake(self.name_without_extension());
        let new_path = self.path.replace(self.name_without_extension(), &new_name);
        Self::new(new_path, self.extension)
    }
    pub fn to_dist(
        &self,
        src_root: &str,
        dist_root: &str,
        dist_extension: impl Into<Extension>,
    ) -> Self {
        let dist_extension = dist_extension.into();
        let dist_path = Extension::replace(
            &self.path.replacen(src_root, dist_root, 1),
            &self.extension,
            &dist_extension,
        );
        Self {
            path: dist_path,
            extension: dist_extension,
        }
    }
}
#[cfg(test)]
mod path_structure_tests {
    use super::*;
    #[test]
    fn 構造体のパスがディレクトリでも親のパスの文字列を返す() {
        let sut = PathStructure::new("./project/src/lib/common/", "rs");

        let result = sut.parent_str();

        assert_eq!(result, "./project/src/lib/");
    }
    #[test]
    fn 親のパスの文字列を返す() {
        let sut = PathStructure::new("./project/src/lib/common/util.rs", "rs");

        let result = sut.parent_str();

        assert_eq!(result, "./project/src/lib/common/");
    }
    #[test]
    fn ルートの指定にルートより上のパスがあってもルート配下のディレクトリのみを返す() {
        let sut = PathStructure::new("./project/src/lib/common/util.rs", "rs");

        let result = sut.all_child_dirs("./project/src");

        assert_eq!(result, vec!["project/src/lib/", "project/src/lib/common/"]);
    }
    #[test]
    fn ルート配下のディレクトリを返す() {
        let sut = PathStructure::new("./src/lib/common/util.rs", "rs");

        let result = sut.all_child_dirs("./src");

        assert_eq!(result, vec!["src/lib/", "src/lib/common/"]);
    }
    #[test]
    fn パスのルートを変更する() {
        let sut = PathStructure::new("./src/main.rs", "rs");

        let result = sut.to_dist("./src", "./dist", Extension::Go);

        assert_eq!(result, PathStructure::new("./dist/main.go", "go"));
    }

    #[test]
    fn パスの名前をsnake_caseに変更する() {
        let sut = PathStructure::new("./src/chain-case.rs", "rs");

        let result = sut.to_snake_path();

        assert_eq!(result, PathStructure::new("./src/chain_case.rs", "rs"));
    }
}
