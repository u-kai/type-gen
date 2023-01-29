use std::{collections::BTreeSet, mem::replace, path::Path};

use crate::filedatas::extension::Extension;

pub struct RootPath<P>
where
    P: AsRef<Path>,
{
    root: String,
    childrens: Vec<P>,
}

pub struct DirectoryConvertor<P>
where
    P: AsRef<Path>,
{
    source: RootPath<P>,
    extension: Extension,
}
impl<P> DirectoryConvertor<P>
where
    P: AsRef<Path>,
{
    pub fn new(source: RootPath<P>, extension: Extension) -> Self {
        Self { source, extension }
    }
    pub fn convert_dirs<'a>(&'a self, dist_root: &'a str) -> impl Iterator<Item = String> + '_ {
        self.source
            .childrens
            .iter()
            .filter_map(|p| {
                if !self.is_this_extension(p) {
                    return None;
                }
                Some(
                    self.replace_src_to_dist(p, dist_root)?
                        .replace(p.as_ref().file_name()?.to_str()?, ""),
                )
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
    pub fn convert_files<'a>(
        &'a self,
        dist_root: &'a str,
        target_extension: Extension,
    ) -> impl Iterator<Item = String> + '_ {
        self.source.childrens.iter().filter_map(move |p| {
            if !self.is_this_extension(p) {
                return None;
            }
            let path = self.replace_src_to_dist(p, dist_root)?;
            Some(Extension::repalace(
                &path,
                &self.extension,
                &target_extension,
            ))
        })
    }
    fn replace_src_to_dist<'a>(&self, src: &P, dist_root: &'a str) -> Option<String> {
        let path = src.as_ref();
        let path = path
            .as_os_str()
            .to_str()?
            .replace(self.source.root.as_str(), dist_root);
        Some(path)
    }
    fn is_this_extension(&self, p: &P) -> bool {
        let path = p.as_ref();
        if let Some(Some(extension)) = path.extension().map(|p| p.to_str()) {
            extension == self.extension.to_str()
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::filedatas::extension::Extension;

    #[test]
    fn ディレクトリの一覧を別のディレクトリ一覧に変換する() {
        let source = RootPath {
            childrens: vec![
                "./src/main.rs",
                "./src/lib/test.rs",
                "./src/lib/child.rs",
                "./src/docs/README.md",
            ],
            root: "src".to_string(),
        };

        let source_extension = Extension::Rs;

        let sut = DirectoryConvertor::new(source, source_extension);
        let mut result = sut.convert_dirs("dist");
        assert_eq!(result.next().unwrap(), "./dist/".to_string());
        assert_eq!(result.next().unwrap(), "./dist/lib/".to_string());
    }
    #[test]
    fn ファイルパスの一覧を別のファイルパスの一覧に変換する() {
        let source = RootPath {
            childrens: vec!["./src/main.rs", "./src/lib/test.rs", "./src/docs/README.md"],
            root: "src".to_string(),
        };

        let source_extension = Extension::Rs;

        let sut = DirectoryConvertor::new(source, source_extension);
        let mut result = sut.convert_files("dist", Extension::Go);
        assert_eq!(result.next().unwrap(), "./dist/main.go".to_string(),);
        assert_eq!(result.next().unwrap(), "./dist/lib/test.go".to_string(),);
        assert_eq!(result.next(), None);
    }
}
