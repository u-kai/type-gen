use std::{fs::read_to_string, path::Path};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FileToFileConfig {
    pub(crate) src: String,
    pub(crate) dist: String,
}

impl FileToFileConfig {
    pub fn new(src: impl Into<String>, dist: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            dist: dist.into(),
        }
    }
    pub fn from_file(file_path: impl AsRef<Path>) -> Self {
        let file_path = file_path.as_ref();
        let file =
            read_to_string(file_path).expect(&format!("{:#?} is not found", file_path.to_str()));
        serde_json::from_str(&file).unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
