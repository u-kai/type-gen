use npc::convertor::NamingPrincipalConvertor;

use super::type_name::TypeName;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PropertyKey {
    original: String,
    converted: Option<String>,
}
impl PropertyKey {
    pub fn to_type_name(&self, parent_type_name: &TypeName) -> TypeName {
        TypeName::new(format!(
            "{}{}",
            parent_type_name.as_str(),
            NamingPrincipalConvertor::new(&self.as_str()).to_pascal()
        ))
    }
    pub fn as_str(&self) -> &str {
        if self.converted.is_some() {
            self.converted.as_ref().unwrap()
        } else {
            &self.original
        }
    }
    pub fn is_rename(&self) -> bool {
        self.converted.is_some()
    }
    pub fn as_original_str(&self) -> &str {
        &self.original
    }
}
fn containe_cannot_use_char(str: &str) -> bool {
    str.contains(|c| match c {
        ':' | ';' | '#' | '$' | '%' | '&' | '~' | '=' | '|' | '\"' | '\'' | '{' | '}' | '?'
        | '!' | '<' | '>' | '[' | ']' | '*' | '^' => true,
        _ => false,
    })
}
fn replace_cannot_use_char(str: &str) -> String {
    str.replace(
        |c| match c {
            ':' | ';' | '#' | '$' | '%' | '&' | '~' | '=' | '|' | '\"' | '\'' | '{' | '}' | '?'
            | '!' | '<' | '>' | '[' | ']' | '*' | '^' => true,
            _ => false,
        },
        "",
    )
}
impl<I> From<I> for PropertyKey
where
    I: Into<String>,
{
    fn from(source: I) -> Self {
        let original = source.into();
        let converted = if containe_cannot_use_char(&original) {
            Some(replace_cannot_use_char(&original))
        } else {
            None
        };
        Self {
            original,
            converted,
        }
    }
}
