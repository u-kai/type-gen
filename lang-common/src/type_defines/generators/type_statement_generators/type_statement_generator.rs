use crate::{
    type_defines::generators::{
        mapper::LangTypeMapper, type_define_generator::TypeStatementGenerator,
    },
    types::type_name::TypeName,
};

use super::{
    alias_type_statement_generator::CustomizableAliasTypeStatementGenerator,
    composite_type_statement_generator::CustomizableCompositeTypeStatementGenerator,
};

// pub struct CustomizableTypeStatementGenerator<F1, F2>
// where
//     F1: Fn(&str, &TypeName, String) -> String,
//     F2: Fn(&str, &TypeName, String) -> String,
// {
//     alias_generator: CustomizableAliasTypeStatementGenerator<F1>,
//     composite_generator: CustomizableCompositeTypeStatementGenerator<F2>,
// }
// impl<F1, F2> CustomizableTypeStatementGenerator<F1, F2>
// where
//     F1: Fn(&str, &TypeName, String) -> String,
//     F2: Fn(&str, &TypeName, String) -> String,
// {
//     pub fn new(
//         alias_generator: CustomizableAliasTypeStatementGenerator<F1>,
//         composite_generator: CustomizableCompositeTypeStatementGenerator<F2>,
//     ) -> Self {
//         Self {
//             alias_generator,
//             composite_generator,
//         }
//     }
// }
// impl<F1, F2> TypeStatementGenerator for CustomizableTypeStatementGenerator<F1, F2>
// where
//     F1: Fn(&str, &TypeName, String) -> String,
//     F2: Fn(&str, &TypeName, String) -> String,
// {
//     fn generate_case_alias<M: LangTypeMapper>(
//         &self,
//         alias_type: &crate::types::structures::AliasTypeStructure,
//         mapper: &M,
//     ) -> String {
//         let statement = mapper.case_property_type(&alias_type.property_type);
//         self.alias_generator
//             .generate_type_define(&alias_type.name, statement)
//     }
//     fn generate_case_composite(
//         &self,
//         type_name: &TypeName,
//         properties_statement: String,
//     ) -> String {
//         self.composite_generator
//             .generate_type_define(type_name, properties_statement)
//     }
// }
// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::{
//         type_defines::generators::{
//             mapper::fake_mapper::FakeLangTypeMapper,
//             type_statement_generators::composite_type_statement_generator::default_concat_fn,
//         },
//         types::{
//             primitive_type::PrimitiveType, property_type::PropertyType,
//             structures::AliasTypeStructure, type_name::TypeName,
//         },
//     };

//     #[test]
//     fn test_type_statement_generator_case_simple() {
//         let type_identify = "struct";
//         let composite_generator =
//             CustomizableCompositeTypeStatementGenerator::new(type_identify, default_concat_fn);

//         let alias_type_identify = "type";
//         let alias_generator = CustomizableAliasTypeStatementGenerator::default();
//         let generator =
//             CustomizableTypeStatementGenerator::new(alias_generator, composite_generator);

//         let type_name: TypeName = "Test".into();
//         let alias_tobe = "type Test = String".to_string();
//         let alias_type_structure = AliasTypeStructure::new(
//             type_name.clone(),
//             PropertyType::Primitive(PrimitiveType::String),
//         );
//         let mapper = FakeLangTypeMapper;
//         assert_eq!(
//             generator.generate_case_alias(&alias_type_structure, &mapper),
//             alias_tobe.to_string()
//         );

//         let property_statements = "id:usize".to_string();
//         let composite_tobe = "struct Test {id:usize}";
//         assert_eq!(
//             generator.generate_case_composite(&type_name, property_statements),
//             composite_tobe.to_string()
//         );
//     }
// }
