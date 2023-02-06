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
}
impl Extension {
    pub fn to_str(&self) -> &'static str {
        self.clone().into()
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
    fn 拡張子の変換機能は拡張子と同じ名前のファイルであっても拡張子だけが置換される() {
        let path = "json.json";
        let result = Extension::replace(path, &Extension::Json, &Extension::Rs);

        assert_eq!(result, "json.rs");
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
