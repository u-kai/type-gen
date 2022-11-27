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
        println!("writed done");
        println!("writed file path is {}", dist_file);
        println!("writed content is {}", writed_content);
    }
}

#[cfg(test)]
mod test_rust_typedefine_dist_file_detail {
    use super::*;
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
