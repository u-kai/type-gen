use description_generator::{
    customizable::declare_part_generator::{
        CompositeTypeDeclareConvertor, CustomizableAliasTypeDeclareGenerator,
        CustomizableCompositeTypeDeclareGenerator, CustomizableDeclarePartGenerator,
    },
    type_description_generator::DeclarePartGenerator,
};
use npc::fns::to_camel;
use structure::parts::type_name::TypeName;

use super::mapper::GoMapper;

pub struct GoDeclarePartGeneratorBuilder {
    generator: GoDeclarePartGenerator,
}
impl GoDeclarePartGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            generator: GoDeclarePartGenerator::new(),
        }
    }
    pub fn build(self) -> GoDeclarePartGenerator {
        self.generator
    }
    pub fn private_all(mut self) -> Self {
        struct PrivateConvertor {}
        impl CompositeTypeDeclareConvertor for PrivateConvertor {
            fn convert(
                &self,
                acc: Option<String>,
                composite_type: &structure::composite_type_structure::CompositeTypeStructure,
            ) -> Option<String> {
                let Some(acc) = acc else {
                    return None
                };
                let type_name = composite_type.type_name();
                Some(acc.replace(&type_name.valid_lang_str(), &to_private(type_name)))
            }
        }
        let convertor = PrivateConvertor {};
        self.generator
            .inner
            .change_composite_generator()
            .add_description_convertor(Box::new(convertor));
        self
    }
    pub fn pub_all(mut self) -> Self {
        struct PubConvertor {}
        impl CompositeTypeDeclareConvertor for PubConvertor {
            fn convert(
                &self,
                acc: Option<String>,
                composite_type: &structure::composite_type_structure::CompositeTypeStructure,
            ) -> Option<String> {
                let Some(acc) = acc else {
                    return None
                };
                let type_name = composite_type.type_name();
                Some(acc.replace(&to_private(type_name), &type_name.valid_lang_str()))
            }
        }
        let convertor = PubConvertor {};
        self.generator
            .inner
            .change_composite_generator()
            .add_description_convertor(Box::new(convertor));
        self
    }
}
pub struct GoDeclarePartGenerator {
    inner: CustomizableDeclarePartGenerator<
        GoMapper,
        fn(&str, &TypeName, String) -> String,
        fn(&str, &TypeName, String) -> String,
    >,
}
fn to_private(type_name: &TypeName) -> String {
    to_camel(&type_name.valid_lang_str())
}
impl GoDeclarePartGenerator {
    pub fn new() -> Self {
        Self {
            inner: CustomizableDeclarePartGenerator::new(
                CustomizableAliasTypeDeclareGenerator::new("type", Self::concat_go_alias),
                CustomizableCompositeTypeDeclareGenerator::new("struct", Self::concat_go_composite),
            ),
        }
    }
    fn concat_go_alias(identify: &str, type_name: &TypeName, type_: String) -> String {
        format!("{} {} {}", identify, type_name.valid_lang_str(), type_)
    }
    fn concat_go_composite(identify: &str, type_name: &TypeName, type_: String) -> String {
        format!(
            "type {} {} {{\n{}}}",
            type_name.valid_lang_str(),
            identify,
            type_
        )
    }
}
impl DeclarePartGenerator for GoDeclarePartGenerator {
    type Mapper = GoMapper;
    fn generate_case_composite(
        &self,
        composite_type: &structure::composite_type_structure::CompositeTypeStructure,
        properties_statement: String,
    ) -> String {
        self.inner
            .generate_case_composite(composite_type, properties_statement)
    }
    fn generate_case_alias(
        &self,
        alias_type: &structure::alias_type_structure::AliasTypeStructure,
        mapper: &Self::Mapper,
    ) -> String {
        self.inner.generate_case_alias(alias_type, mapper)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use structure::{
        alias_type_structure::AliasTypeStructure, composite_type_structure::CompositeTypeStructure,
        parts::property_type::property_type_factories::make_string_type,
    };

    use super::*;
    #[test]
    fn 不正な文字列は型名に指定できない() {
        let type_name = TypeName::new("Test:invalidName");
        let composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let property_part = "    id int\n".to_string();

        let sut = GoDeclarePartGenerator::new();
        let result = sut.generate_case_composite(&composite_type, property_part);
        assert_eq!(
            result,
            r#"type TestInvalidName struct {
    id int
}"#
        );
    }
    #[test]
    fn private_allをすると名前はパスカルケースで出力されない() {
        let type_name = TypeName::new("Test");
        let composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let property_part = "    id int\n".to_string();

        let sut = GoDeclarePartGenerator::new();
        let result = sut.generate_case_composite(&composite_type, property_part);
        assert_eq!(
            result,
            r#"type Test struct {
    id int
}"#
        );
    }
    #[test]
    fn 文字列型のalias型定義の作成() {
        let sut = GoDeclarePartGenerator::new();
        let mapper = GoMapper {};
        let alias = AliasTypeStructure::new("Test", make_string_type());
        let result = sut.generate_case_alias(&alias, &mapper);
        println!("{}", result);
        assert_eq!(result, "type Test string");
    }
    #[test]
    fn pubの設定をするとtypenameがパスカルケースになる() {
        let type_name = TypeName::new("Test");
        let composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let property_part = "    id int\n".to_string();

        let sut = GoDeclarePartGeneratorBuilder::new().pub_all().build();
        let result = sut.generate_case_composite(&composite_type, property_part);
        println!("{}", result);
        assert_eq!(
            result,
            r#"type Test struct {
    id int
}"#
        );
    }
}
