use std::path::Path;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Extension {
    Rs,
    Txt,
    Java,
    Go,
    Ts,
    Py,
    Json,
    Empty,
}
impl Extension {
    pub fn to_str(&self) -> &'static str {
        self.clone().into()
    }
    pub fn to_filepath(path_str: &str, extension: impl Into<Extension>) -> String {
        let path: &Path = path_str.as_ref();
        let extension = extension.into();
        // case file
        if let Some(Some(_extension)) = path.extension().map(|e| e.to_str()) {
            return Self::change_extension(path, extension).unwrap();
        }

        // case dir
        if path_str.get(path_str.len() - 1..path_str.len()) == Some("/") {
            return format!("{}.{}", &path_str[..path_str.len() - 1], extension.to_str());
        }
        format!("{}.{}", path_str, extension.to_str())
    }
    pub fn remove_extension(path: impl AsRef<Path>) -> String {
        if let Some(Some(extension)) = path.as_ref().extension().map(|f| f.to_str()) {
            path.as_ref()
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .replace(&format!(".{}", extension), "")
        } else {
            path.as_ref()
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string()
        }
    }
    pub fn change_extension(path: impl AsRef<Path>, dist: impl Into<Extension>) -> Option<String> {
        let extension = path.as_ref().extension()?.to_str()?;
        Some(path.as_ref().file_name()?.to_str()?.replace(
            &format!(".{}", extension),
            &format!(".{}", dist.into().to_str()),
        ))
    }
    pub fn replace(path: &str, source: &Extension, dist: &Extension) -> String {
        path.replace(
            &format!(".{}", source.to_str()),
            &format!(".{}", dist.to_str()),
        )
    }
    pub fn is_this_extension(&self, path: impl AsRef<Path>) -> bool {
        path.as_ref()
            .extension()
            .map(|ex| Some(self.to_str()) == ex.to_str())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::Extension;

    #[test]
    fn pathの末尾がスラッシュではないものを指定したファイルに変更する() {
        let path = "dir/json";
        let result = Extension::to_filepath(path, "rs");

        assert_eq!(result, "dir/json.rs");
    }
    #[test]
    fn ディレクトリを指定したファイルに変更する() {
        let path = "dir/json/";
        let result = Extension::to_filepath(path, "rs");

        assert_eq!(result, "dir/json.rs");
    }
    #[test]
    fn ファイルを指定したファイルに変更する() {
        let path = "json.json";
        let result = Extension::to_filepath(path, "rs");

        assert_eq!(result, "json.rs");
    }
    #[test]
    fn 拡張子の変換機能は拡張子と同じ名前のファイルであっても拡張子だけが変更される() {
        let path = "json.json";
        let result = Extension::replace(path, &Extension::Json, &Extension::Rs);

        assert_eq!(result, "json.rs");
    }
    #[test]
    fn 拡張子を削除する() {
        let path = "json.json";
        let result = Extension::remove_extension(path);

        assert_eq!(result, "json");
    }
}
impl From<&str> for Extension {
    fn from(s: &str) -> Self {
        match s {
            "rs" => Extension::Rs,
            "txt" => Extension::Txt,
            "java" => Extension::Java,
            "go" => Extension::Go,
            "ts" => Extension::Ts,
            "py" => Extension::Py,
            "json" => Extension::Json,
            "" => Extension::Empty,
            _ => panic!("not impl extension {}", s),
        }
    }
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
            Extension::Empty => "",
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
            Self::Empty => "",
        }
    }
}
