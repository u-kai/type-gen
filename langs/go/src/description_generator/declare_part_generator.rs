use description_generator::{
    customizable::declare_part_generator::{
        CompositeTypeDeclareConvertor, CustomizableAliasTypeDeclareGenerator,
        CustomizableCompositeTypeDeclareGenerator, CustomizableDeclarePartGenerator,
    },
    type_description_generator::DeclarePartGenerator,
};
use structure::parts::type_name::TypeName;

use super::mapper::GoMapper;

pub struct GoDeclarePartGenerator {
    inner: CustomizableDeclarePartGenerator<
        GoMapper,
        fn(&str, &TypeName, String) -> String,
        fn(&str, &TypeName, String) -> String,
    >,
}

impl GoDeclarePartGenerator {
    fn concat_go_alias(identify: &str, type_name: &TypeName, type_: String) -> String {
        format!("{} {} {}", identify, type_name.as_str(), type_)
    }
    fn concat_go_composite(identify: &str, type_name: &TypeName, type_: String) -> String {
        format!("type {} {} {{\n{}}}", type_name.as_str(), identify, type_)
    }
    fn new() -> Self {
        Self {
            inner: CustomizableDeclarePartGenerator::new(
                CustomizableAliasTypeDeclareGenerator::new("type", Self::concat_go_alias),
                CustomizableCompositeTypeDeclareGenerator::new("struct", Self::concat_go_composite),
            ),
        }
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
        alias_type_structure::AliasTypeStructure,
        composite_type_structure::CompositeTypeStructure,
        parts::property_type::property_type_factories::{make_primitive_type, make_string_type},
    };

    use super::*;
    #[test]
    fn カスタム型のalias型定義の作成() {
        let sut = GoDeclarePartGenerator::new();
        let mapper = GoMapper {};
        let alias = AliasTypeStructure::new("Test", make_string_type());
        let result = sut.generate_case_alias(&alias, &mapper);
        println!("{}", result);
        assert_eq!(result, "type Test string");
    }
    #[test]
    fn idという整数型のプロパティを持つ型定義の作成() {
        let type_name = TypeName::new("Test");
        let composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let property_part = "    id int\n".to_string();

        let sut = GoDeclarePartGenerator::new();
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
