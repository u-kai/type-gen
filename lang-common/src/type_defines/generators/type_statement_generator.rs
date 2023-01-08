use crate::types::type_name::TypeName;

use super::{mapper::LangTypeMapper, type_define_generator::TypeStatementGenerator};

pub struct CustomizableTypeStatementGenerator<F1, F2>
where
    F1: Fn(&str, &TypeName, String) -> String,
    F2: Fn(&str, &TypeName, String) -> String,
{
    alias_generator: CustomizableAliasTypeStatementGenerator<F1>,
    composite_generator: CustomizableCompositeTypeStatementGenerator<F2>,
}
impl<F1, F2> CustomizableTypeStatementGenerator<F1, F2>
where
    F1: Fn(&str, &TypeName, String) -> String,
    F2: Fn(&str, &TypeName, String) -> String,
{
    pub fn new(
        alias_generator: CustomizableAliasTypeStatementGenerator<F1>,
        composite_generator: CustomizableCompositeTypeStatementGenerator<F2>,
    ) -> Self {
        Self {
            alias_generator,
            composite_generator,
        }
    }
}
impl<F1, F2> TypeStatementGenerator for CustomizableTypeStatementGenerator<F1, F2>
where
    F1: Fn(&str, &TypeName, String) -> String,
    F2: Fn(&str, &TypeName, String) -> String,
{
    //type Mapper = M;
    fn generate_case_alias<M: LangTypeMapper>(
        &self,
        alias_type: &crate::types::structures::AliasTypeStructure,
        mapper: &M,
    ) -> String {
        let statement = mapper.case_property_type(&alias_type.property_type);
        self.alias_generator
            .generate_type_define(&alias_type.name, statement)
    }
    fn generate_case_composite(
        &self,
        type_name: &TypeName,
        properties_statement: String,
    ) -> String {
        self.composite_generator
            .generate_type_define(type_name, properties_statement)
    }
}
pub struct CustomizableAliasTypeStatementGenerator<F>
where
    F: Fn(&str, &TypeName, String) -> String,
{
    alias_type_identify: &'static str,
    concat_fn: F,
}
impl<F> CustomizableAliasTypeStatementGenerator<F>
where
    F: Fn(&str, &TypeName, String) -> String,
{
    pub fn new(alias_type_identify: &'static str, concat_fn: F) -> Self {
        Self {
            alias_type_identify,
            concat_fn,
        }
    }
    fn generate_type_define(
        &self,
        type_name: &crate::types::type_name::TypeName,
        properties_statement: String,
    ) -> String {
        let f = &self.concat_fn;
        f(self.alias_type_identify, type_name, properties_statement)
    }
}
pub struct CustomizableCompositeTypeStatementGenerator<F>
where
    F: Fn(&str, &TypeName, String) -> String,
{
    type_identify: &'static str,
    concat_fn: F,
}
impl<F> CustomizableCompositeTypeStatementGenerator<F>
where
    F: Fn(&str, &TypeName, String) -> String,
{
    pub fn new(type_identify: &'static str, concat_fn: F) -> Self {
        Self {
            type_identify,
            concat_fn,
        }
    }
    fn generate_type_define(
        &self,
        type_name: &crate::types::type_name::TypeName,
        properties_statement: String,
    ) -> String {
        let f = &self.concat_fn;
        f(self.type_identify, type_name, properties_statement)
    }
}
//impl<F, M> TypeStatementGenerator<M> for CustomizableCompositeTypeStatementGenerator<F>
//where
//    F: Fn(&str, &TypeName, String) -> String,
//    M: LangTypeMapper,
//{
//    fn generate_case_composite(
//        &self,
//        type_name: &crate::types::type_name::TypeName,
//        properties_statement: String,
//    ) -> String {
//        let f = &self.concat_fn;
//        f(self.type_identify, type_name, properties_statement)
//    }
//    fn generate_case_alias(
//        &self,
//        primitive_type: &crate::types::structures::AliasTypeStructure,
//        mapper: &M,
//    ) -> String {
//        String::new()
//    }
//}
//impl<M:LangTypeMapper> Default for CustomizableTypeStatementGenerator<fn(&str,&TypeName,String)->String,M> {
//    fn default() -> Self {
//        CustomizableTypeStatementGenerator { type_identify: "class", concat_fn:  m: () }
//    }
//}
#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        type_defines::generators::mapper::fake_mapper::FakeLangTypeMapper,
        types::{
            primitive_type::PrimitiveType, property_type::PropertyType,
            structures::AliasTypeStructure, type_name::TypeName,
        },
    };

    fn default_concat_fn(
        type_identify: &str,
        type_name: &TypeName,
        property_statements: String,
    ) -> String {
        format!(
            "{} {} {{{}}}",
            type_identify,
            type_name.as_str(),
            property_statements
        )
    }
    fn default_alias_concat_fn(
        alias_type_identify: &str,
        type_name: &TypeName,
        property_statements: String,
    ) -> String {
        format!(
            "{} {} = {}",
            alias_type_identify,
            type_name.as_str(),
            property_statements
        )
    }
    #[test]
    fn test_type_statement_generator_case_simple() {
        let type_identify = "struct";
        let composite_generator =
            CustomizableCompositeTypeStatementGenerator::new(type_identify, default_concat_fn);

        let alias_type_identify = "type";
        let alias_generator = CustomizableAliasTypeStatementGenerator::new(
            alias_type_identify,
            default_alias_concat_fn,
        );
        let generator =
            CustomizableTypeStatementGenerator::new(alias_generator, composite_generator);

        let type_name: TypeName = "Test".into();
        let alias_tobe = "type Test = String".to_string();
        let alias_type_structure =
            AliasTypeStructure::new(type_name, PropertyType::Primitive(PrimitiveType::String));
        let mapper = FakeLangTypeMapper;
        assert_eq!(
            generator.generate_case_alias(&alias_type_structure, &mapper),
            alias_tobe.to_string()
        );

        let property_statements = "id:usize".to_string();
        let composite_tobe = "struct Test {id:usize}";
        //        assert_eq!(
        //            generator.generate_case_composite(&type_name, property_statements),
        //            composite_tobe.to_string()
        //        );
    }
    #[test]
    fn test_case_alias_type_simple() {
        let property_statement = "String".to_string();
        let alias_type_identify = "type";
        let type_name: TypeName = "Test".into();
        let tobe = "type Test = String".to_string();
        let generator = CustomizableAliasTypeStatementGenerator::new(
            alias_type_identify,
            default_alias_concat_fn,
        );
        assert_eq!(
            generator.generate_type_define(&type_name, property_statement),
            tobe
        );
    }
    #[test]
    fn test_case_composite_type_simple() {
        let property_statements = "id:usize".to_string();
        let type_identify = "struct";
        let type_name: TypeName = "Test".into();
        let tobe = "struct Test {id:usize}";
        let generator =
            CustomizableCompositeTypeStatementGenerator::new(type_identify, default_concat_fn);
        assert_eq!(
            generator.generate_type_define(&type_name, property_statements),
            tobe.to_string()
        );

        let type_identify = "class";
        let property_statements = "id:usize".to_string();
        let type_name: TypeName = "Test".into();
        let tobe = "class Test {id:usize}";
        let generator =
            CustomizableCompositeTypeStatementGenerator::new(type_identify, default_concat_fn);
        assert_eq!(
            generator.generate_type_define(&type_name, property_statements),
            tobe.to_string()
        )
    }
    #[test]
    fn test_type_define_type_use_convertor() {
        let property_statements = "    id:usize".to_string();
        let type_identify = "struct";
        let type_name: TypeName = "Test".into();
        fn concat_identity_and_name_and_property_statement(
            type_identify: &str,
            type_name: &TypeName,
            property_statements: String,
        ) -> String {
            format!(
                "{} {} {{\n{}\n}}",
                type_identify,
                type_name.as_str(),
                property_statements
            )
        }
        let tobe = "struct Test {
    id:usize
}";
        let generator = CustomizableCompositeTypeStatementGenerator::new(
            type_identify,
            concat_identity_and_name_and_property_statement,
        );
        assert_eq!(
            generator.generate_type_define(&type_name, property_statements),
            tobe.to_string()
        );
    }
}
