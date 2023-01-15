use description_generator::{
    customizable::{
        property_part_convertors::{AddLastSideConvertor, AddLeftSideConvertor},
        property_part_generator::{Convertor, CustomizablePropertyDescriptionGenerator},
    },
    type_description_generator::PropertyPartGenerator,
};
use npc::fns::{is_snake, to_snake};

use super::mapper::RustMapper;

pub struct RustPropertyPartGenerator {
    generator: CustomizablePropertyDescriptionGenerator<fn(String, String) -> String, RustMapper>,
}
impl RustPropertyPartGenerator {
    const INDENT: &'static str = "    ";
    const NEXT_LINE: &'static str = ",\n";
    pub fn new() -> Self {
        fn rust_property_concat(key: String, type_: String) -> String {
            format!("{}: {}", key, type_)
        }
        let mut result = Self {
            generator: CustomizablePropertyDescriptionGenerator::new(rust_property_concat),
        };
        result.add_default_convertors();
        result
    }
    fn add_default_convertors(&mut self) {
        let mut add_space_convertor = AddLeftSideConvertor::new(Self::INDENT);
        add_space_convertor.set_all();
        let rename_convertor = RustRenameConverotor::new();
        let add_serde_rename_convertor = RustAddSerdeRenameConverotor::new();
        let mut add_last_side_convertor = AddLastSideConvertor::new(Self::NEXT_LINE);
        add_last_side_convertor.set_all();
        self.generator
            .add_property_key_convertor(Box::new(rename_convertor));
        self.generator
            .add_statement_convertor(Box::new(add_serde_rename_convertor));
        self.generator
            .add_statement_convertor(Box::new(add_space_convertor));
        self.generator
            .add_statement_convertor(Box::new(add_last_side_convertor));
    }
}
impl PropertyPartGenerator<RustMapper> for RustPropertyPartGenerator {
    fn generate(
        &self,
        type_name: &structure::parts::type_name::TypeName,
        property_key: &structure::parts::property_key::PropertyKey,
        property_type: &structure::parts::property_type::PropertyType,
        mapper: &RustMapper,
    ) -> String {
        self.generator
            .generate(type_name, property_key, property_type, mapper)
    }
}
#[cfg(test)]
mod tests {
    use crate::description_generator::{
        mapper::RustMapper, property_part_generator::RustPropertyPartGenerator,
    };
    use description_generator::type_description_generator::PropertyPartGenerator;
    use structure::parts::{
        property_key::PropertyKey, property_type::property_type_factories::make_usize_type,
        type_name::TypeName,
    };
    #[test]
    fn test_case_not_use_str() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id:value".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let generator = RustPropertyPartGenerator::new();
        let tobe = format!("    #[serde(rename = \"id:value\")]\n    idvalue: usize,\n",);
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_primitive_all_none_additional() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let generator = RustPropertyPartGenerator::new();
        let tobe = "    id: usize,\n".to_string();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
    }
}

struct RustRenameJudger {
    reserved_words: RustReservedWords,
}
impl RustRenameJudger {
    fn new() -> Self {
        Self {
            reserved_words: RustReservedWords::new(),
        }
    }
    fn do_need_rename(&self, word: &str) -> bool {
        self.is_rust_principal(word) || self.is_cannot_use_word(word)
    }
    fn is_rust_principal(&self, word: &str) -> bool {
        !is_snake(word)
    }
    fn is_cannot_use_word(&self, word: &str) -> bool {
        Self::containe_cannot_use_char(word)
            || self.reserved_words.is_reserved_keywords(word)
            || self.reserved_words.is_strict_keywords(word)
    }
    fn containe_cannot_use_char(str: &str) -> bool {
        str.contains(Self::cannot_use_char)
    }
    fn cannot_use_char(c: char) -> bool {
        match c {
            ':' | ';' | '#' | '$' | '%' | '&' | '~' | '=' | '|' | '\"' | '\'' | '{' | '}' | '?'
            | '!' | '<' | '>' | '[' | ']' | '*' | '^' => true,
            _ => false,
        }
    }
}
pub struct RustRenameConverotor {
    judger: RustRenameJudger,
}
impl RustRenameConverotor {
    pub fn new() -> Self {
        Self {
            judger: RustRenameJudger::new(),
        }
    }
}
impl Convertor<RustMapper> for RustRenameConverotor {
    fn convert(
        &self,
        acc: &mut String,
        _: &structure::parts::type_name::TypeName,
        property_key: &structure::parts::property_key::PropertyKey,
        _: &structure::parts::property_type::PropertyType,
        _: &RustMapper,
    ) -> () {
        fn cannot_use_char(c: char) -> bool {
            match c {
                ':' | ';' | '#' | '$' | '%' | '&' | '~' | '=' | '|' | '\"' | '\'' | '{' | '}'
                | '?' | '!' | '<' | '>' | '[' | ']' | '*' | '^' => true,
                _ => false,
            }
        }
        fn replace_cannot_use_char(str: &str) -> String {
            str.replace(cannot_use_char, "")
        }
        if self.judger.do_need_rename(property_key.as_str()) {
            *acc = format!(
                "{}",
                to_snake(&replace_cannot_use_char(property_key.as_str()))
            )
        }
    }
}
pub struct RustAddSerdeRenameConverotor {
    judger: RustRenameJudger,
}
impl RustAddSerdeRenameConverotor {
    pub fn new() -> Self {
        Self {
            judger: RustRenameJudger::new(),
        }
    }
}
impl Convertor<RustMapper> for RustAddSerdeRenameConverotor {
    fn convert(
        &self,
        acc: &mut String,
        _: &structure::parts::type_name::TypeName,
        property_key: &structure::parts::property_key::PropertyKey,
        _: &structure::parts::property_type::PropertyType,
        _: &RustMapper,
    ) -> () {
        if self.judger.do_need_rename(property_key.as_str()) {
            *acc = format!("#[serde(rename = \"{}\")]\n{}", property_key.as_str(), acc)
        }
    }
}
#[cfg(test)]
mod rust_rename_convertor_tests {
    use description_generator::customizable::property_part_generator::Convertor;
    use structure::parts::{
        property_key::PropertyKey, property_type::property_type_factories::make_usize_type,
        type_name::TypeName,
    };

    use crate::description_generator::{
        mapper::RustMapper, property_part_generator::RustAddSerdeRenameConverotor,
    };

    #[test]
    fn test_case_not_use_str() {
        let mut acc = "idvalue:usize".to_string();
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id:value".into();
        let property_type = make_usize_type();
        let tobe = format!("#[serde(rename = \"id:value\")]\nidvalue:usize");
        let mapper = RustMapper;
        let convertor = RustAddSerdeRenameConverotor::new();
        convertor.convert(&mut acc, &type_name, &property_key, &property_type, &mapper);
        assert_eq!(acc, tobe);
    }
}

#[derive(Debug, Clone)]
pub struct RustReservedWords {
    reserved: [&'static str; 45],
    strict: [&'static str; 7],
}

impl RustReservedWords {
    pub fn new() -> Self {
        let reserved = [
            "as", "async", "await", "break", "continue", "else", "enum", "false", "true", "fn",
            "const", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
            "pub", "ref", "return", "static", "struct", "trait", "true", "type", "unsafe", "where",
            "while", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
            "try", "typeof", "unsized", "virtual", "yield",
        ];
        let strict = ["extern", "Self", "self", "use", "crate", "_", "super"];
        Self { reserved, strict }
    }
    pub fn is_reserved_keywords(&self, word: &str) -> bool {
        self.reserved.contains(&word)
    }
    pub fn is_strict_keywords(&self, word: &str) -> bool {
        self.strict.contains(&word)
    }
}

#[cfg(test)]
mod test_rust_reserved_words {
    use super::RustReservedWords;

    #[test]
    fn test_get_or_origin() {
        let reserved_words = RustReservedWords::new();
        assert_eq!(reserved_words.is_reserved_keywords("type"), true);
        assert_eq!(reserved_words.is_strict_keywords("super"), true);
        assert_eq!(reserved_words.is_reserved_keywords("data"), false);
        assert_eq!(reserved_words.is_strict_keywords("data"), false);
    }
}