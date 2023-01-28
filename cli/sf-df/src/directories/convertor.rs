use std::path::Path;

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
    pub fn convert(&self, dist_root: &str, target_extension: Extension) -> Vec<String> {
        self.source
            .childrens
            .iter()
            .filter_map(|p| {
                let path = p.as_ref();
                let extension = path.extension()?.to_str()?;
                if extension != self.extension.to_str() {
                    return None;
                }
                Some(
                    path.as_os_str()
                        .to_str()?
                        .replace(self.source.root.as_str(), dist_root)
                        .replace(self.extension.to_str(), target_extension.to_str()),
                )
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::filedatas::extension::Extension;

    #[test]
    fn ディレクトリの一覧を別のディレクトリ一覧に変換する() {
        let source = RootPath {
            childrens: vec!["./src/main.rs", "./src/lib/test.rs", "./src/docs/README.md"],
            root: "src".to_string(),
        };

        let source_extension = Extension::Rs;

        let sut = DirectoryConvertor::new(source, source_extension);
        let result = sut.convert("dist", Extension::Go);

        assert_eq!(
            result,
            vec![
                "./dist/main.go".to_string(),
                "./dist/lib/test.go".to_string(),
            ]
        )
    }
}
