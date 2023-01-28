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
impl Extension {
    pub fn to_str(&self) -> &'static str {
        self.clone().into()
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
