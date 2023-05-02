use description_generator::{
    customizable::{
        declare_part_convetors::{
            AddHeaderConvertor, BlackListConvertor, ToDeclarePartConvertor, WhiteListConvertor,
        },
        declare_part_generator::{
            CustomizableAliasTypeDeclareGenerator, CustomizableCompositeTypeDeclareGenerator,
            CustomizableDeclarePartGenerator,
        },
    },
    type_description_generator::DeclarePartGenerator,
};
use structure::parts::type_name::TypeName;

use super::mapper::RustMapper;
impl DeclarePartGenerator for RustDeclarePartGenerator {
    type Mapper = RustMapper;
    fn generate_case_alias(
        &self,
        alias_type: &structure::alias_type_structure::AliasTypeStructure,
        mapper: &Self::Mapper,
    ) -> String {
        self.inner.generate_case_alias(alias_type, mapper)
    }
    fn generate_case_composite(
        &self,
        composite_type: &structure::composite_type_structure::CompositeTypeStructure,
        properties_statement: String,
    ) -> String {
        self.inner
            .generate_case_composite(composite_type, properties_statement)
    }
}
impl RustDeclarePartGenerator {
    fn new() -> Self {
        fn alias_concat(identify: &str, type_name: &TypeName, description: String) -> String {
            format!(
                "{} {} = {};",
                identify,
                type_name.valid_lang_str(),
                description
            )
        }
        fn concat_composite_description_use_curly_bracket(
            identify: &str,
            type_name: &TypeName,
            property_descriptions: String,
        ) -> String {
            format!(
                "{} {} {{\n{}}}",
                identify,
                type_name.valid_lang_str(),
                property_descriptions
            )
        }
        RustDeclarePartGenerator {
            inner: CustomizableDeclarePartGenerator::new(
                CustomizableAliasTypeDeclareGenerator::new("type", alias_concat),
                CustomizableCompositeTypeDeclareGenerator::new(
                    "struct",
                    concat_composite_description_use_curly_bracket,
                ),
            ),
        }
    }
    fn change_alias_generator(
        &mut self,
    ) -> &mut CustomizableAliasTypeDeclareGenerator<RustMapper, fn(&str, &TypeName, String) -> String>
    {
        self.inner.change_alias_generator()
    }
    fn change_composite_generator(
        &mut self,
    ) -> &mut CustomizableCompositeTypeDeclareGenerator<fn(&str, &TypeName, String) -> String> {
        self.inner.change_composite_generator()
    }
}

pub struct RustDeclarePartGeneratorBuilder {
    generator: RustDeclarePartGenerator,
}
impl RustDeclarePartGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            generator: RustDeclarePartGenerator::new(),
        }
    }
    pub fn all_attrs_with_serde(self, attrs: Vec<impl Into<String>>) -> Self {
        let mut attrs = attrs.into_iter().map(|s| s.into()).collect::<Vec<_>>();
        attrs.push("serde::Deserialize".to_string());
        attrs.push("serde::Serialize".to_string());
        self.all_attrs(attrs)
    }
    pub fn all_comment(mut self, comment: impl Into<String>) -> Self {
        let mut convertor = AddHeaderConvertor::new(format!("// {}", comment.into()));
        convertor.all();
        self.generator
            .change_composite_generator()
            .add_description_convertor(convertor.to_declare_part());
        self.generator
            .change_alias_generator()
            .add_description_convertor(Box::new(convertor));
        self
    }
    pub fn set_all_derive_with_serde(self, derives: Vec<impl Into<String>>) -> Self {
        let mut derives = derives.into_iter().map(|s| s.into()).collect::<Vec<_>>();
        derives.push("serde::Deserialize".to_string());
        derives.push("serde::Serialize".to_string());
        self.set_all_derive(derives)
    }
    pub fn set_all_derive(mut self, derives: Vec<impl Into<String>>) -> Self {
        let derive_description = format!(
            "#[derive({})]",
            derives
                .into_iter()
                .map(|s| {
                    let s: String = s.into();
                    s
                })
                .reduce(|acc, cur| { format!("{},{}", acc, cur) })
                .unwrap_or_default()
        );
        let mut convertor = AddHeaderConvertor::new(derive_description);
        convertor.all();
        self.generator
            .change_composite_generator()
            .add_description_convertor(convertor.to_declare_part());
        self
    }
    pub fn pub_all(self) -> Self {
        self.pub_all_alias().pub_all_composite()
    }
    pub fn pub_all_alias(mut self) -> Self {
        let mut convertor = AddHeaderConvertor::new("pub ");
        convertor.all();
        self.generator
            .change_alias_generator()
            .add_type_identify_convertor(convertor.to_declare_part());
        self
    }
    pub fn pub_all_composite(mut self) -> Self {
        let mut convertor = AddHeaderConvertor::new("pub ");
        convertor.all();
        self.generator
            .change_composite_generator()
            .add_type_identify_convertor(convertor.to_declare_part());
        self
    }
    pub fn all_attrs(mut self, attrs: Vec<impl Into<String>>) -> Self {
        let mut attrs = attrs
            .into_iter()
            .map(|s| s.into())
            .fold(String::new(), |acc, cur| format!("{}#[{}]\n", acc, cur));
        attrs.pop();
        let mut convertor = AddHeaderConvertor::new(attrs);
        convertor.all();
        self.generator
            .change_composite_generator()
            .add_description_convertor(convertor.to_declare_part());
        self.generator
            .change_alias_generator()
            .add_description_convertor(convertor.to_declare_part());
        self
    }
    pub fn set_whitelist(mut self, list: Vec<impl Into<String>>) -> Self {
        let mut convertor = WhiteListConvertor::new();
        list.into_iter().for_each(|v| convertor.add(v));
        self.generator
            .change_composite_generator()
            .add_description_convertor(convertor.to_declare_part());
        self.generator
            .change_alias_generator()
            .add_description_convertor(convertor.to_declare_part());
        self
    }
    pub fn set_blacklist(mut self, list: Vec<impl Into<String>>) -> Self {
        let mut convertor = BlackListConvertor::new();
        list.into_iter().for_each(|v| convertor.add(v));
        self.generator
            .change_composite_generator()
            .add_description_convertor(convertor.to_declare_part());
        self.generator
            .change_alias_generator()
            .add_description_convertor(convertor.to_declare_part());
        self
    }
    pub fn change_alias_generator(
        &mut self,
    ) -> &mut CustomizableAliasTypeDeclareGenerator<RustMapper, fn(&str, &TypeName, String) -> String>
    {
        self.generator.change_alias_generator()
    }
    pub fn change_composite_generator(
        &mut self,
    ) -> &mut CustomizableCompositeTypeDeclareGenerator<fn(&str, &TypeName, String) -> String> {
        self.generator.change_composite_generator()
    }
    pub fn build(self) -> RustDeclarePartGenerator {
        self.generator
    }
}
pub struct RustDeclarePartGenerator {
    inner: CustomizableDeclarePartGenerator<
        RustMapper,
        fn(&str, &TypeName, String) -> String,
        fn(&str, &TypeName, String) -> String,
    >,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use structure::{
        alias_type_structure::AliasTypeStructure,
        composite_type_structure::CompositeTypeStructure,
        parts::{property_type::property_type_factories::make_string_type, type_name::TypeName},
    };

    use crate::description_generator::mapper::RustMapper;

    use super::*;
    #[test]
    fn test_case_all_attrs() {
        let type_name: TypeName = "Test".into();
        let composite_type = CompositeTypeStructure::new(type_name.clone(), BTreeMap::new());
        let generator = RustDeclarePartGeneratorBuilder::new()
            .all_attrs(vec!["allow(notuse)", "target=win"])
            .build();
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n")),
            "#[allow(notuse)]
#[target=win]
struct Test {
    id: usize,
}"
        );
    }
    #[test]
    fn test_case_set_whitelist() {
        let type_name: TypeName = "Test".into();
        let composite_type = CompositeTypeStructure::new(type_name.clone(), BTreeMap::new());
        let generator = RustDeclarePartGeneratorBuilder::new()
            .set_whitelist(vec!["User"])
            .pub_all()
            .build();
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n"),),
            ""
        );
        let mapper = RustMapper;
        let primitive_type = AliasTypeStructure::new(type_name, make_string_type());
        assert_eq!(generator.generate_case_alias(&primitive_type, &mapper,), "");
        let type_name: TypeName = "User".into();
        let composite_type = CompositeTypeStructure::new(type_name.clone(), BTreeMap::new());
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n")),
            "pub struct User {
    id: usize,
}"
        );
    }
    #[test]
    fn test_case_set_blacklist() {
        let type_name: TypeName = "Test".into();
        let composite_type = CompositeTypeStructure::new(type_name.clone(), BTreeMap::new());
        let generator = RustDeclarePartGeneratorBuilder::new()
            .set_blacklist(vec!["Test"])
            .pub_all()
            .build();
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n"),),
            ""
        );
        let mapper = RustMapper;
        let primitive_type = AliasTypeStructure::new(type_name, make_string_type());
        assert_eq!(generator.generate_case_alias(&primitive_type, &mapper,), "");
        let type_name: TypeName = "User".into();
        let composite_type = CompositeTypeStructure::new(type_name.clone(), BTreeMap::new());
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n")),
            "pub struct User {
    id: usize,
}"
        );
    }
    #[test]
    fn test_case_add_comment() {
        let type_name: TypeName = "Test".into();
        let composite_type = CompositeTypeStructure::new(type_name.clone(), BTreeMap::new());
        let generator = RustDeclarePartGeneratorBuilder::new()
            .all_comment("this is comment")
            .build();
        let tobe = r#"// this is comment
struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n"),),
            tobe
        );
        let mapper = RustMapper;
        let primitive_type = AliasTypeStructure::new(type_name, make_string_type());
        let tobe = "// this is comment
type Test = String;";
        assert_eq!(
            generator.generate_case_alias(&primitive_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_composite_add_derive_and_pub() {
        let type_name: TypeName = "Test".into();
        let composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let generator = RustDeclarePartGeneratorBuilder::new()
            .set_all_derive(vec!["Debug", "Clone"])
            .pub_all_composite()
            .build();
        let tobe = r#"#[derive(Debug,Clone)]
pub struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n"),),
            tobe
        );
    }
    #[test]
    fn test_case_composite_add_derive() {
        let type_name: TypeName = "Test".into();
        let composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let generator = RustDeclarePartGeneratorBuilder::new()
            .set_all_derive(vec!["Debug", "Clone"])
            .build();
        let tobe = r#"#[derive(Debug,Clone)]
struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n"),),
            tobe
        );
    }
    #[test]
    fn test_case_add_pub_and_derive() {
        let type_name: TypeName = "Test".into();
        let composite_type = CompositeTypeStructure::new(type_name.clone(), BTreeMap::new());
        let generator = RustDeclarePartGeneratorBuilder::new()
            .pub_all_alias()
            .pub_all_composite()
            .set_all_derive(vec!["Debug", "Clone"])
            .build();
        let tobe = r#"#[derive(Debug,Clone)]
pub struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n"),),
            tobe
        );
    }
    #[test]
    fn 不正な文字列は型名に指定できない() {
        let type_name: TypeName = "Test:invalidName".into();
        let mapper = RustMapper;
        let generator = RustDeclarePartGeneratorBuilder::new()
            .set_all_derive(vec!["Debug", "Clone"])
            .build();

        let primitive_type = AliasTypeStructure::new(type_name, make_string_type());
        let tobe = format!("type TestInvalidName = String;");
        assert_eq!(
            generator.generate_case_alias(&primitive_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn alias_typeにはderiveを設定できない() {
        let type_name: TypeName = "Test".into();
        let mapper = RustMapper;
        let generator = RustDeclarePartGeneratorBuilder::new()
            .set_all_derive(vec!["Debug", "Clone"])
            .build();

        let primitive_type = AliasTypeStructure::new(type_name, make_string_type());
        let tobe = format!("type Test = String;");
        assert_eq!(
            generator.generate_case_alias(&primitive_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_add_all_pub() {
        let type_name: TypeName = "Test".into();
        let mapper = RustMapper;
        let composite_type = CompositeTypeStructure::new(type_name.clone(), BTreeMap::new());
        let generator = RustDeclarePartGeneratorBuilder::new().pub_all().build();
        let tobe = r#"pub struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n"),),
            tobe
        );
        let primitive_type = AliasTypeStructure::new(type_name, make_string_type());
        let tobe = format!("pub type Test = String;");
        assert_eq!(
            generator.generate_case_alias(&primitive_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_add_pub() {
        let type_name: TypeName = "Test".into();
        let mapper = RustMapper;
        let composite_type = CompositeTypeStructure::new(type_name.clone(), BTreeMap::new());
        let generator = RustDeclarePartGeneratorBuilder::new()
            .pub_all_composite()
            .pub_all_alias()
            .build();
        let tobe = r#"pub struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n"),),
            tobe
        );
        let primitive_type = AliasTypeStructure::new(type_name, make_string_type());
        let tobe = format!("pub type Test = String;");
        assert_eq!(
            generator.generate_case_alias(&primitive_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_composite_add_pub() {
        let type_name: TypeName = "Test".into();
        let composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let generator = RustDeclarePartGeneratorBuilder::new()
            .pub_all_composite()
            .build();
        let tobe = r#"pub struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n"),),
            tobe
        );
    }
    #[test]
    fn test_case_alias_add_pub() {
        let type_name: TypeName = "Test".into();
        let mapper = RustMapper;
        let primitive_type = AliasTypeStructure::new(type_name, make_string_type());
        let generator = RustDeclarePartGeneratorBuilder::new()
            .pub_all_alias()
            .build();
        let tobe = format!("pub type Test = String;");
        assert_eq!(
            generator.generate_case_alias(&primitive_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_alias_all_none_additional() {
        let type_name: TypeName = "Test".into();
        let mapper = RustMapper;
        let primitive_type = AliasTypeStructure::new(type_name, make_string_type());
        let generator = RustDeclarePartGeneratorBuilder::new().build();
        let tobe = format!("type Test = String;");
        assert_eq!(
            generator.generate_case_alias(&primitive_type, &mapper,),
            tobe
        );
    }
    #[test]
    fn test_case_composite_all_none_additional() {
        let type_name: TypeName = "Test".into();
        let composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let generator = RustDeclarePartGeneratorBuilder::new().build();
        let tobe = r#"struct Test {
    id: usize,
}"#;
        assert_eq!(
            generator.generate_case_composite(&composite_type, format!("    id: usize,\n"),),
            tobe
        );
    }
}
