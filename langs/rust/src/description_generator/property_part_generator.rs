use description_generator::{
    customizable::{
        property_part_convertors::{
            AddHeaderConvertor, AddLastSideConvertor, AddLeftSideConvertor, BlackListConvertor,
            ToOptionalConvertor, WhiteListConvertor,
        },
        property_part_generator::{
            Convertor, CustomizablePropertyDescriptionGenerator, DescriptionConvertor,
        },
    },
    type_description_generator::PropertyPartGenerator,
};
use npc::fns::{is_snake, to_snake};

use super::mapper::RustMapper;
pub enum RustVisibility {
    Private,
    Public,
    PublicSuper,
    PublicSelf,
    PublicCrate,
}
impl RustVisibility {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Private => "",
            Self::Public => "pub ",
            Self::PublicSuper => "pub(super) ",
            Self::PublicSelf => "pub(self) ",
            Self::PublicCrate => "pub(crate) ",
        }
    }
    fn from_str(str: &str) -> Result<Self, String> {
        match str {
            "pub" | "public" | "Pub" | "Public" | "export" => Ok(Self::Public),
            "" | "private" | "Private" => Ok(Self::Private),
            "pub(self)" | "pub (self)" | "pub self" => Ok(Self::PublicSelf),
            "pub(super)" | "pub (super)" | "pub super" => Ok(Self::PublicSuper),
            "pub(crate)" | "pub (crate)" | "pub crate" => Ok(Self::PublicCrate),
            _ => Err(format!("{} is not define rust visibility", str)),
        }
    }
}

impl Default for RustVisibility {
    fn default() -> Self {
        Self::Private
    }
}
impl<T> From<T> for RustVisibility
where
    T: Into<String>,
{
    fn from(str: T) -> Self {
        let str: String = str.into();
        RustVisibility::from_str(&str).unwrap()
    }
}
pub struct RustPropertyPartGeneratorBuilder {
    generator: RustPropertyPartGenerator,
}
impl RustPropertyPartGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            generator: RustPropertyPartGenerator::new(),
        }
    }
    pub fn build(self) -> RustPropertyPartGenerator {
        let mut generator = self.generator;
        generator.add_default_convertors();
        generator
    }

    pub fn change_property_generator(
        &mut self,
    ) -> &mut CustomizablePropertyDescriptionGenerator<fn(String, String) -> String, RustMapper>
    {
        self.generator.change_property_generator()
    }

    pub fn set_whitelist_with_keys(mut self, list: Vec<impl Into<String>>) -> Self {
        let mut white_list = WhiteListConvertor::new();
        list.into_iter()
            .for_each(|s| white_list.add_match_property_key(s));
        self.generator
            .generator
            .add_statement_convertor(Box::new(white_list));
        self
    }
    pub fn set_blacklist_with_keys(mut self, list: Vec<impl Into<String>>) -> Self {
        let mut black_list = BlackListConvertor::new();
        list.into_iter()
            .for_each(|s| black_list.add_match_property_key(s));
        self.generator
            .generator
            .add_statement_convertor(Box::new(black_list));
        self
    }
    pub fn all_attrs(mut self, attrs: Vec<impl Into<String>>) -> Self {
        let attrs = attrs
            .into_iter()
            .map(|s| s.into())
            .reduce(|acc, cur| format!("{},{}", acc, cur))
            .unwrap();
        let mut convertor = AddHeaderConvertor::new(format!("#[{}]", attrs));
        convertor.set_all();
        self.generator
            .generator
            .add_statement_convertor(Box::new(convertor));
        self
    }
    pub fn all_comment(mut self, comment: &'static str) -> Self {
        let mut convertor = AddHeaderConvertor::new(format!("// {}", comment));
        convertor.set_all();
        self.generator
            .generator
            .add_statement_convertor(Box::new(convertor));
        self
    }
    pub fn all_optional(mut self) -> Self {
        let mut convertor = ToOptionalConvertor::new();
        convertor.set_all();
        self.generator
            .generator
            .add_property_type_convertor(Box::new(convertor));
        self
    }
    pub fn all_visibility(mut self, visibility: RustVisibility) -> Self {
        let mut convertor = AddLeftSideConvertor::new(visibility.as_str());
        convertor.set_all();
        self.generator
            .generator
            .add_statement_convertor(Box::new(convertor));
        self
    }
}

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
        let result = Self {
            generator: CustomizablePropertyDescriptionGenerator::new(rust_property_concat),
        };
        result
    }
    fn change_property_generator(
        &mut self,
    ) -> &mut CustomizablePropertyDescriptionGenerator<fn(String, String) -> String, RustMapper>
    {
        &mut self.generator
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
        mapper::RustMapper,
        property_part_generator::{RustPropertyPartGeneratorBuilder, RustVisibility},
    };
    use description_generator::type_description_generator::PropertyPartGenerator;
    use structure::parts::{
        property_key::PropertyKey, property_type::property_type_factories::make_usize_type,
        type_name::TypeName,
    };
    #[test]
    fn test_case_set_whitelist_with_key() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let generator = RustPropertyPartGeneratorBuilder::new()
            .set_whitelist_with_keys(vec!["test"])
            .build();
        let tobe = format!("");
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "test".into();
        let property_type = make_usize_type();
        let tobe = format!("    test: usize,\n");
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_set_blacklist_with_key() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let generator = RustPropertyPartGeneratorBuilder::new()
            .set_blacklist_with_keys(vec!["id"])
            .build();
        let tobe = format!("");
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "test".into();
        let property_type = make_usize_type();
        let tobe = format!("    test: usize,\n");
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_add_all_attrs() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let attrs = vec!["allow(notuse)", "target=(mac)"];
        let generator = RustPropertyPartGeneratorBuilder::new()
            .all_attrs(attrs)
            .build();
        let tobe = format!("    #[allow(notuse),target=(mac)]\n    id: usize,\n",);
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_add_all_comment() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let comment = "this is comment";
        let generator = RustPropertyPartGeneratorBuilder::new()
            .all_comment(comment)
            .build();
        let tobe = format!("    // {}\n    id: usize,\n", comment);
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_optional() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let generator = RustPropertyPartGeneratorBuilder::new()
            .all_optional()
            .build();
        let tobe = format!("    id: Option<usize>,\n",);
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_pub() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id:value".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let generator = RustPropertyPartGeneratorBuilder::new()
            .all_visibility(RustVisibility::Public)
            .build();
        let tobe = format!("    #[serde(rename = \"id:value\")]\n    pub idvalue: usize,\n",);
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id:value".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let generator = RustPropertyPartGeneratorBuilder::new()
            .all_visibility(RustVisibility::PublicSuper)
            .build();
        let tobe =
            format!("    #[serde(rename = \"id:value\")]\n    pub(super) idvalue: usize,\n",);
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_not_use_str() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id:value".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let generator = RustPropertyPartGeneratorBuilder::new().build();
        let tobe = format!("    #[serde(rename = \"id:value\")]\n    idvalue: usize,\n",);
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_reserved_words() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "type".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let generator = RustPropertyPartGeneratorBuilder::new().build();
        let tobe = "    #[serde(rename = \"type\")]\n    r#type: usize,\n".to_string();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_strict_reserved_words() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "self".into();
        let property_type = make_usize_type();
        let mapper = RustMapper;
        let generator = RustPropertyPartGeneratorBuilder::new().build();
        let tobe = "    #[serde(rename = \"self\")]\n    self_: usize,\n".to_string();
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
        let generator = RustPropertyPartGeneratorBuilder::new().build();
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
                | '?' | '!' | '<' | '>' | '[' | ']' | '*' | '^' | '(' | ')' => true,
                _ => false,
            }
        }
        fn replace_cannot_use_char(str: &str) -> String {
            str.replace(cannot_use_char, "")
        }
        if self
            .judger
            .reserved_words
            .is_reserved_keywords(property_key.as_str())
        {
            *acc = format!("r#{}", acc);
            return;
        }
        if self
            .judger
            .reserved_words
            .is_strict_keywords(property_key.as_str())
        {
            *acc = format!("{}_", acc);
            return;
        }
        if self.judger.do_need_rename(property_key.as_str()) {
            *acc = format!("{}", to_snake(&replace_cannot_use_char(acc)))
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
impl DescriptionConvertor<RustMapper> for RustAddSerdeRenameConverotor {
    fn convert(
        &self,
        acc: Option<String>,
        _: &structure::parts::type_name::TypeName,
        property_key: &structure::parts::property_key::PropertyKey,
        _: &structure::parts::property_type::PropertyType,
        _: &RustMapper,
    ) -> Option<String> {
        if self.judger.do_need_rename(property_key.as_str()) {
            if let Some(acc) = acc {
                return Some(format!(
                    "#[serde(rename = \"{}\")]\n{}",
                    property_key.as_str(),
                    acc
                ));
            }
        }
        acc
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
