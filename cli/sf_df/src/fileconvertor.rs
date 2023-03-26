use std::{collections::BTreeSet, fs::read_to_string, path::Path};

use npc::fns::{to_snake, to_snake_consider_with_wellknown_word};

use crate::{extension::Extension, fileoperator::is_dir};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileStructure {
    content: String,
    path: PathStructure,
}
impl FileStructure {
    pub fn new(content: impl Into<String>, path: PathStructure) -> Self {
        Self {
            content: content.into(),
            path: path,
        }
    }
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let path_structure = PathStructure::from_path(&path);
        let content = read_to_string(&path).unwrap();
        Self {
            content,
            path: path_structure,
        }
    }
    pub fn to_snake_path(self) -> Self {
        Self::new(self.content, self.path.to_snake_path())
    }
    pub fn to_snake_path_consider_with_wellknown_words(self) -> Self {
        Self::new(
            self.content,
            self.path.to_snake_path_consider_with_wellknown_words(),
        )
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
    pub fn to(
        &self,
        dist_root: &str,
        dist_extension: impl Into<Extension>,
        content: impl Into<String>,
    ) -> Self {
        let dist = self.path.to(dist_root, dist_extension);
        Self::new(content, dist)
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
impl FileStructure {
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
        let sut = FileStructure::new("fn main(){}", PathStructure::new("src/main.rs", "rs"));

        assert_eq!(sut.name_without_extension(), "main");
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
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let extension = Extension::from(
            path.extension()
                .expect(&format!("{:#?} is not has extension", path))
                .to_str()
                .unwrap(),
        );
        let path = path.to_str().unwrap().to_string();
        Self { path, extension }
    }
    pub fn to(&self, dist_path: &str, extension: impl Into<Extension>) -> Self {
        let extension: Extension = extension.into();
        let dist_path = if is_dir(dist_path) {
            format!(
                "{}/{}.{}",
                dist_path,
                self.name_without_extension(),
                extension.to_str()
            )
        } else {
            dist_path.to_string()
        };
        Self {
            path: dist_path,
            extension: extension,
        }
    }
    pub fn parent_str(&self) -> String {
        let path: &Path = self.path.as_ref();
        if let Some(Some(filename)) = path.file_name().map(|f| f.to_str()) {
            if filename == path.to_str().unwrap() {
                return "./".to_string();
            }
            let mut result = self.path.replace(filename, "");
            if result.len() > 2 && &result[result.len() - 2..] == "//" {
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
    pub fn to_snake_path_consider_with_wellknown_words(self) -> Self {
        let new_name = to_snake_consider_with_wellknown_word(self.name_without_extension());
        let new_path = self.path.replace(self.name_without_extension(), &new_name);
        Self::new(new_path, self.extension)
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
        let dist_root = Self::make_dist_root(src_root, dist_root);
        let dist_path = Extension::replace(
            &self.path.replacen(src_root, &dist_root, 1),
            &self.extension,
            &dist_extension,
        );
        Self {
            path: dist_path,
            extension: dist_extension,
        }
    }
    // case src_root end to "/", for example "./","/","src/" or empty, dist root end have to "/"
    fn make_dist_root(src_root: &str, dist_root: &str) -> String {
        if dist_root.get(dist_root.len() - 1..dist_root.len()) != Some("/")
            && (src_root.len() == 0
                || src_root.get(src_root.len() - 1..src_root.len()) == Some("/"))
        {
            format!("{}/", dist_root)
        } else {
            dist_root.to_string()
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
    fn 空文字の親のパスは空文字を返す() {
        let sut = PathStructure::new("a", "");

        let result = sut.parent_str();

        assert_eq!(result, "./");
    }
    #[test]
    fn 親のパスがない場合はカレントディレクトリを返す() {
        let sut = PathStructure::new("util.rs", "rs");

        let result = sut.parent_str();

        assert_eq!(result, "./");
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
        let sut = PathStructure::new("./main.rs", "rs");

        let result = sut.to_dist("./", "./dist", Extension::Go);

        assert_eq!(result, PathStructure::new("./dist/main.go", "go"));
    }
    #[test]
    fn 指定した拡張子とディレクトリにpathを変換する() {
        let src = PathStructure::from_path("test/test.json");
        let dist_parent = "dist";
        let sut = src.to(dist_parent, "rs");

        assert_eq!(sut.path_str(), "dist/test.rs");
    }

    #[test]
    fn パスの名前をsnake_caseに変更する() {
        let sut = PathStructure::new("./src/chain-case.rs", "rs");

        let result = sut.to_snake_path();

        assert_eq!(result, PathStructure::new("./src/chain_case.rs", "rs"));
    }
    #[test]
    fn path構造体から生成できる() {
        let sut = PathStructure::from_path("./src/main.rs");

        let result = sut.path_str();

        assert_eq!(result, "./src/main.rs");
        assert_eq!(sut.extension, Extension::Rs);
    }
}
