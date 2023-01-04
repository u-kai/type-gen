use lang_common::type_defines::additional_defines::attribute_store::Attribute;

use super::property_generator::RUST_PROPERTY_HEAD_SPACE;

#[derive(Debug, Clone)]
pub struct RustAttribute {
    all: Vec<RustAttributeKind>,
}
impl RustAttribute {
    pub fn new() -> Self {
        Self { all: Vec::new() }
    }
    pub fn add_attribute(&mut self, attr: RustAttributeKind) {
        self.all.push(attr);
    }
    pub fn from_derives(derives: Vec<&'static str>) -> Self {
        let attr = RustAttributeKind::Derives(derives);
        let mut result = Self::new();
        result.add_attribute(attr);
        result
    }
}
impl From<RustAttributeKind> for RustAttribute {
    fn from(kind: RustAttributeKind) -> Self {
        match kind {
            RustAttributeKind::Original(originals) => {
                let all = originals
                    .split("\n")
                    .map(|ori| RustAttributeKind::Original(ori.to_string()))
                    .collect::<Vec<_>>();
                Self { all }
            }
            _ => Self { all: vec![kind] },
        }
    }
}

impl<I> From<I> for RustAttribute
where
    I: Into<String>,
{
    fn from(str: I) -> Self {
        let str: String = str.into();
        let all = str
            .split("\n")
            .map(|ori| RustAttributeKind::Original(ori.to_string()))
            .collect::<Vec<_>>();
        Self { all }
    }
}
impl Attribute for RustAttribute {
    fn to_property_define(&self) -> String {
        self.all.iter().fold(String::new(), |acc, cur| {
            format!("{}{}{}\n", acc, RUST_PROPERTY_HEAD_SPACE, cur.to_string())
        })
    }
    fn to_type_define(&self) -> String {
        self.all.iter().fold(String::new(), |acc, cur| {
            format!("{}{}\n", acc, cur.to_string())
        })
    }
}
#[derive(Debug, Clone)]
pub enum RustAttributeKind {
    CfgTest,
    Test,
    Derives(Vec<&'static str>),
    Original(String),
}
impl RustAttributeKind {
    fn to_string(&self) -> String {
        match self {
            Self::CfgTest => "#[cfg(test)]".to_string(),
            Self::Test => "#[test]".to_string(),
            Self::Derives(derives) => {
                let mut derives = derives
                    .iter()
                    .fold(String::new(), |acc, cur| format!("{}{},", acc, cur));
                derives.remove(derives.len() - 1);
                format!("#[derive({})]", derives)
            }
            Self::Original(str) => {
                format!("#[{}]", str)
            }
        }
    }
}
