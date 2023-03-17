use std::{fs::read_to_string, path::Path};

#[derive(serde::Deserialize, serde::Serialize, Debug)]

pub struct FileToFileConfig {
    pub src: String,
    pub dist: String,
}

impl FileToFileConfig {
    pub fn new(src: impl Into<String>, dist: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            dist: dist.into(),
        }
    }
    pub fn from_file(file_path: impl AsRef<Path>) -> Result<Self, FileToFileConfigError> {
        let file_path = file_path.as_ref();
        if let Ok(file) = read_to_string(file_path) {
            match serde_json::from_str(&file) {
                Err(e) => {
                    println!("{:#?}", e);
                    Err(FileToFileConfigError::CanNotSerializeJson(e.to_string()))
                }
                Ok(result) => Ok(result),
            }
        } else {
            Err(FileToFileConfigError::ConfigFileNotFound)
        }
    }
}

#[derive(Debug)]
pub enum FileToFileConfigError {
    ConfigFileNotFound,
    CanNotSerializeJson(String),
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
