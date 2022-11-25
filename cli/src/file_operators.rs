use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy)]
pub enum Extension {
    Rs,
    Txt,
    Java,
    Go,
    Ts,
    Py,
    Json,
}

impl Into<&'static str> for &Extension {
    fn into(self) -> &'static str {
        match self {
            Extension::Rs => "rs",
            Extension::Txt => "txt",
            Extension::Java => "java",
            Extension::Go => "go",
            Extension::Ts => "ts",
            Extension::Py => "py",
            Extension::Json => "json",
        }
    }
}
impl Into<&'static str> for Extension {
    fn into(self) -> &'static str {
        match self {
            Self::Rs => "rs",
            Self::Txt => "txt",
            Self::Java => "java",
            Self::Go => "go",
            Self::Ts => "ts",
            Self::Py => "py",
            Self::Json => "json",
        }
    }
}

pub(crate) struct TypeGenDistFilesWriter<'a> {
    src_all_files: Vec<PathBuf>,
    dist_extension: Extension,
    src: &'a str,
    dist: &'a str,
}

impl<'a> TypeGenDistFilesWriter<'a> {
    pub fn new(src: &'a str, dist: &'a str, dist_extension: Extension) -> Self {
        let src_all_files = all_file_path(src);
        Self {
            src_all_files,
            dist_extension,
            src,
            dist,
        }
    }
    pub fn generate_all_dist_file_path(&self) -> impl Iterator<Item = String> + '_ {
        self.src_all_files
            .iter()
            .map(|dir| {
                let extension = dir.extension().unwrap().to_str().unwrap();
                let dist_extension: &str = self.dist_extension.into();
                let new_filename = dir
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(extension, dist_extension);
                format!(
                    "{}{}",
                    extract_directory_part_from_path(dir).replace(self.src, self.dist),
                    new_filename
                )
            })
            .into_iter()
    }
}
pub(crate) struct TypeGenDistDirectoriesCreator<'a> {
    src: &'a str,
    dist: &'a str,
}

impl<'a> TypeGenDistDirectoriesCreator<'a> {
    pub fn new(src: &'a str, dist: &'a str) -> Self {
        Self { src, dist }
    }
    pub fn create_dist_directories(&self, all_src_files: &Vec<PathBuf>) {
        self.generate_all_dist_directory_path(all_src_files)
            .for_each(|dir| mkdir(dir));
    }
    fn generate_all_dist_directory_path(
        &self,
        all_src_files: &Vec<PathBuf>,
    ) -> impl Iterator<Item = String> + '_ {
        all_src_files
            .into_iter()
            .map(|src| extract_directory_part_from_path(src).replace(self.src, self.dist))
            .flat_map(|dist| split_dirs(dist))
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
}

fn split_dirs(path: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let all_dir = extract_directory_part_from_path(path);
    let mut dir = String::new();
    all_dir
        .split("/")
        .into_iter()
        .filter(|s| *s != "." && *s != "")
        .fold(Vec::new(), |mut acc, s| {
            dir += &format!("{}/", s);
            acc.push(dir.clone());
            acc
        })
        .into_iter()
}
fn all_file_path(root_dir_path: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut all_files = Vec::new();
    let root_dir = fs::read_dir(root_dir_path).unwrap();
    root_dir
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| match entry.file_type() {
            Ok(file_type) => Some((file_type, entry.path())),
            Err(_) => None,
        })
        .for_each(|(file_type, path)| {
            if file_type.is_dir() {
                let mut files = all_file_path(path);
                all_files.append(&mut files);
                return;
            }
            all_files.push(path)
        });
    all_files
}
fn mkdir(path: impl AsRef<Path>) {
    if !path.as_ref().exists() {
        fs::create_dir(path.as_ref()).unwrap();
    }
}
fn extract_directory_part_from_path(path: impl AsRef<Path>) -> String {
    if path.as_ref().is_dir() || path.as_ref().extension().is_none() {
        return path.as_ref().to_str().unwrap().to_string();
    }
    let filename = path.as_ref().file_name().unwrap().to_str().unwrap();
    path.as_ref().to_str().unwrap().replace(filename, "")
}
pub fn mv_files(
    dirs: Vec<impl AsRef<Path>>,
    src: &str,
    dist: &str,
    to_extension: &str,
) -> Vec<String> {
    dirs.into_iter()
        .map(|dir| {
            let dir = dir.as_ref();
            let extension = dir.extension().unwrap().to_str().unwrap();
            let original_filename = dir.file_name().unwrap().to_str().unwrap();
            let new_filename = original_filename.replace(extension, to_extension);
            format!(
                "{}{}",
                extract_directory_part_from_path(dir).replace(src, dist),
                new_filename
            )
        })
        .collect()
}

mod test_file_operations {
    use super::*;
    #[test]
    fn test_mv_files() {
        let paths = vec![
            "./src/test.txt",
            "./src/dir1/test.txt",
            "./src/dir2/test.txt",
        ];
        assert_eq!(
            mv_files(paths, "src", "dist", "rs"),
            vec![
                "./dist/test.rs",
                "./dist/dir1/test.rs",
                "./dist/dir2/test.rs",
            ]
        );
    }
    #[test]
    fn test_get_dir() {
        let path = "./dir1/test.txt";
        assert_eq!(
            extract_directory_part_from_path(path),
            "./dir1/".to_string()
        );
    }
    #[test]
    fn test_split_dirs() {
        let path = "./src/example/child/test.txt";
        assert_eq!(
            split_dirs(path).collect::<Vec<_>>(),
            vec![
                "src/".to_string(),
                "src/example/".to_string(),
                "src/example/child/".to_string(),
            ]
        );
        let path = "./src/example/child/";
        assert_eq!(
            split_dirs(path).collect::<Vec<_>>(),
            vec![
                "src/".to_string(),
                "src/example/".to_string(),
                "src/example/child/".to_string(),
            ]
        );
    }
}
