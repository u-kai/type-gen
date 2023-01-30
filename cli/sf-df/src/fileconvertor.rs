use std::path::Path;

use npc::fns::to_snake;

use crate::extension::Extension;

pub trait FileStructerConvertor {
    fn convert(&self, file: &FileStructer, extension: impl Into<Extension>) -> FileStructer;
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
    pub fn path(&self) -> &str {
        &self.path.path
    }
    pub fn to_dist(
        &self,
        dist_root: impl Into<String>,
        dist_extension: impl Into<Extension>,
        content: impl Into<String>,
    ) -> Self {
        let dist = self.path.to_dist(dist_root, dist_extension);
        Self::new(content, dist)
    }
}
#[cfg(test)]
mod file_structer_tests {
    use super::*;
    #[test]
    fn pathと拡張子が取り除かれたファイル名を返す() {
        let sut = FileStructer::new(
            "fn main(){}",
            PathStructure::new("src", "src/main.rs", "rs"),
        );

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
            FileStructer::new(
                "func main(){}",
                PathStructure::new("src", "src/main.go", "go"),
            ),
            FileStructer::new(
                "func main(){}",
                PathStructure::new("src", "src/lib/lib.go", "go"),
            ),
            FileStructer::new(
                "func main(){}",
                PathStructure::new("src", "src/bin/bin.go", "go"),
            ),
        ];
        let sut = FileConvetor::new(source);
        struct FakeConvertor {}
        impl FileStructerConvertor for FakeConvertor {
            fn convert(&self, f: &FileStructer, e: impl Into<Extension>) -> FileStructer {
                let content = f.content().replace("func", "fn");
                f.to_dist("dist", e, content)
            }
        }
        let convertor = FakeConvertor {};
        let result = sut.convert(Extension::Rs, convertor);
        assert_eq!(
            result,
            vec![
                FileStructer::new(
                    "fn main(){}",
                    PathStructure::new("dist", "dist/main.rs", "rs",)
                ),
                FileStructer::new(
                    "fn main(){}",
                    PathStructure::new("dist", "dist/lib/lib.rs", "rs",)
                ),
                FileStructer::new(
                    "fn main(){}",
                    PathStructure::new("dist", "dist/bin/bin.rs", "rs",)
                ),
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
    #[cfg(not(target_os = "windows"))]
    pub const SEPARATOR: &'static str = "/";
    #[cfg(any(target_os = "windows", feature = "test_win"))]
    pub const SEPARATOR: &'static str = "\\";

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
    pub fn all_dir(&self) -> Vec<String> {
        let all_dir = self.extract_dir();
        let mut dir = String::new();
        all_dir
            .split(Self::SEPARATOR)
            .into_iter()
            .filter(|s| *s != "." && *s != "")
            .fold(Vec::new(), |mut acc, s| {
                dir += &format!("{}{}", s, Self::SEPARATOR);
                acc.push(dir.clone());
                acc
            })
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
        Self::new(self.root, new_path, self.extension)
    }
    pub fn to_dist(
        &self,
        dist_root: impl Into<String>,
        dist_extension: impl Into<Extension>,
    ) -> Self {
        let dist_root = dist_root.into();
        let dist_extension = dist_extension.into();
        let dist_path = Extension::repalace(
            &self.path.replacen(&self.root, &dist_root, 1),
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
#[cfg(test)]
mod path_structure_tests {
    use super::*;
    #[test]
    fn ルート配下のディレクトリを返す() {
        let sut = PathStructure::new("./src", "./src/lib/common/util.rs", "rs");

        let result = sut.all_dir();

        assert_eq!(result, vec!["src/", "src/lib/", "src/lib/common/"]);
    }
    #[test]
    fn パスのルートを変更する() {
        let sut = PathStructure::new("./src", "./src/main.rs", "rs");

        let result = sut.to_dist("./dist", Extension::Go);

        assert_eq!(result, PathStructure::new("./dist", "./dist/main.go", "go"));
    }

    #[test]
    fn パスの名前をsnake_caseに変更する() {
        let sut = PathStructure::new("./src", "./src/chain-case.rs", "rs");

        let result = sut.to_snake_path();

        assert_eq!(
            result,
            PathStructure::new("./src", "./src/chain_case.rs", "rs")
        );
    }
}
