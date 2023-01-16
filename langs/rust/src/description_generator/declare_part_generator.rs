use description_generator::{
    customizable::declare_part_generator::{
        CustomizableAliasTypeDeclareGenerator, CustomizableCompositeTypeDeclareGenerator,
        CustomizableDeclarePartGenerator,
    },
    type_description_generator::DeclarePartGenerator,
};
use structure::parts::type_name::TypeName;

use super::mapper::RustMapper;

pub struct RustDeclarePartGeneratorBuilder {}
impl RustDeclarePartGeneratorBuilder {
    pub fn new() -> Self {
        Self {}
    }
    pub fn build(self) -> RustDeclarePartGenerator {
        let mut generator = RustDeclarePartGenerator::new();
        generator
    }
}
pub struct RustDeclarePartGenerator {
    generator: CustomizableDeclarePartGenerator<
        RustMapper,
        fn(&str, &TypeName, String) -> String,
        fn(&str, &TypeName, String) -> String,
    >,
}
impl RustDeclarePartGenerator {
    fn new() -> Self {
        fn alias_concat(identify: &str, type_name: &TypeName, description: String) -> String {
            format!("{} {} = {};", identify, type_name.as_str(), description)
        }
        fn concat_composite_description_use_curly_bracket(
            identify: &str,
            type_name: &TypeName,
            property_descriptions: String,
        ) -> String {
            format!(
                "{} {} {{\n{}}}",
                identify,
                type_name.as_str(),
                property_descriptions
            )
        }
        RustDeclarePartGenerator {
            generator: CustomizableDeclarePartGenerator::new(
                CustomizableAliasTypeDeclareGenerator::new("type", alias_concat),
                CustomizableCompositeTypeDeclareGenerator::new(
                    "struct",
                    concat_composite_description_use_curly_bracket,
                ),
            ),
        }
    }
}
impl DeclarePartGenerator for RustDeclarePartGenerator {
    type Mapper = RustMapper;
    fn generate_case_alias(
        &self,
        alias_type: &structure::alias_type_structure::AliasTypeStructure,
        mapper: &Self::Mapper,
    ) -> String {
        self.generator.generate_case_alias(alias_type, mapper)
    }
    fn generate_case_composite(
        &self,
        composite_type: &structure::composite_type_structure::CompositeTypeStructure,
        properties_statement: String,
    ) -> String {
        self.generator
            .generate_case_composite(composite_type, properties_statement)
    }
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
