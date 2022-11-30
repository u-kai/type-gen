use std::{
    fs::{read_to_string, File, OpenOptions},
    io::Write,
    path::Path,
};

use crate::from_src_files::fs_operators::dist_writer::TypeDefineDistFileDetail;
use npc::convertor::NamingPrincipalConvertor;
pub struct RustTypeDefineDistFileDetail {
    dependencies: Vec<&'static str>,
}
impl RustTypeDefineDistFileDetail {
    pub fn new() -> Self {
        Self {
            dependencies: vec!["serde::{Deserialize,Serialize}", "serde_json::Value"],
        }
    }
    fn get_parent_filename(dist_file: impl AsRef<Path>) -> Option<String> {
        fn get_writed_filename(dist_file: impl AsRef<Path>) -> Option<String> {
            Some(dist_file.as_ref().file_name()?.to_str()?.to_string())
        }
        Some(
            dist_file
                .as_ref()
                .to_str()?
                .replace(&format!("/{}", get_writed_filename(&dist_file)?), ".rs"),
        )
    }
}
impl TypeDefineDistFileDetail for RustTypeDefineDistFileDetail {
    fn add_content(&self, content: String) -> String {
        self.dependencies
            .iter()
            .fold(content, |acc, cur| format!("use {};\n{}", cur, acc))
    }
    fn filename(&self, original: String) -> String {
        NamingPrincipalConvertor::new(&original).to_snake()
    }
    fn finaly(&self, dist_file: String, writed_content: String) {
        let mod_name: &Path = dist_file.as_ref();
        let mod_name = mod_name
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".rs", "");
        if let Some(parent_filename) = Self::get_parent_filename(&dist_file) {
            let path: &Path = parent_filename.as_ref();
            let mut file = if path.exists() {
                OpenOptions::new()
                    .append(true)
                    .read(true)
                    .open(path)
                    .expect(&format!(
                        "path is {:?}, mod_name {} , \ncontent {}",
                        path, mod_name, writed_content
                    ))
            } else {
                File::create(path).expect(&format!(
                    "path is {:?}, mod_name {} , \ncontent {}",
                    path, mod_name, writed_content
                ))
            };
            let write_mod_content = format!("pub mod {};\n", mod_name);
            match read_to_string(path) {
                Ok(str) if !str.contains(&write_mod_content) => {
                    file.write_all(write_mod_content.as_bytes())
                        .expect(&format!(
                            "path is {:?}, mod_name {} , \ncontent {}",
                            path, mod_name, writed_content
                        ));
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod test_rust_typedefine_dist_file_detail {
    use super::*;
    #[test]
    fn test_get_parent_rs() {
        let write_filename = "src/requests/api.rs";
        let tobe = "src/requests.rs";
        assert_eq!(
            RustTypeDefineDistFileDetail::get_parent_filename(write_filename).unwrap(),
            tobe.to_string()
        )
    }
    #[test]
    fn test_filename() {
        let old_filename = "test-rust.rs";
        let tobe = "test_rust.rs";
        assert_eq!(
            RustTypeDefineDistFileDetail::new().filename(old_filename.to_string()),
            tobe.to_string()
        )
    }
    #[test]
    fn test_add_content() {
        let content = "test";
        let tobe = format!(
            "use serde_json::Value;\nuse serde::{{Deserialize,Serialize}};\n{}",
            content
        );
        let detail = RustTypeDefineDistFileDetail::new();
        assert_eq!(detail.add_content(content.to_string()), tobe);
    }
}
