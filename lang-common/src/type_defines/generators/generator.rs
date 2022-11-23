use crate::{
    type_defines::{
        additional_defines::additional_statement::AdditionalStatement, type_define::TypeDefine,
    },
    types::{
        property_key::PropertyKey,
        property_type::PropertyType,
        structures::{PrimitiveTypeStructure, TypeStructure},
        type_name::TypeName,
    },
};

use super::mapper::LangTypeMapper;

pub struct TypeDefineGenerator<T, P, M, A>
where
    T: TypeStructureGenerator,
    P: PropertyStatementGenerator,
    M: LangTypeMapper,
    A: AdditionalStatement,
{
    type_statement_generator: T,
    property_statement_generator: P,
    mapper: M,
    additional_statement: A,
}
impl<T, P, M, A> TypeDefineGenerator<T, P, M, A>
where
    T: TypeStructureGenerator,
    P: PropertyStatementGenerator,
    M: LangTypeMapper,
    A: AdditionalStatement,
{
    pub fn new(
        type_statement_generator: T,
        property_statement_generator: P,
        mapper: M,
        additional_statement: A,
    ) -> Self {
        Self {
            type_statement_generator,
            property_statement_generator,
            mapper,
            additional_statement,
        }
    }
    pub fn generate(&self, structure: TypeStructure) -> TypeDefine {
        match structure {
            TypeStructure::Composite(composite) => {
                let properties_statement =
                    composite
                        .properties
                        .iter()
                        .fold(String::new(), |acc, (k, v)| {
                            format!(
                                "{}{}",
                                acc,
                                self.property_statement_generator.generate(
                                    &composite.name,
                                    k,
                                    v,
                                    &self.mapper,
                                    &self.additional_statement
                                )
                            )
                        });
                self.type_statement_generator.generate_case_composite(
                    &composite.name,
                    properties_statement,
                    &self.additional_statement,
                )
            }
            TypeStructure::Primitive(primitive) => self
                .type_statement_generator
                .generate_case_primitive(&primitive, &self.mapper, &self.additional_statement),
        }
    }
}

pub trait TypeStructureGenerator {
    const TYPE_PREFIX: &'static str = "class";
    fn generate_case_composite<A: AdditionalStatement>(
        &self,
        type_name: &TypeName,
        properties_statement: String,
        additional_statement: &A,
    ) -> String;
    fn generate_case_primitive<M: LangTypeMapper, A: AdditionalStatement>(
        &self,
        primitive_type: &PrimitiveTypeStructure,
        mapper: &M,
        additional_statement: &A,
    ) -> String;
}
pub trait PropertyStatementGenerator {
    fn generate<M: LangTypeMapper, A: AdditionalStatement>(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
        property_type: &PropertyType,
        mapper: &M,
        additional_statement: &A,
    ) -> String;
}
#[cfg(test)]
pub mod fakes {
    use crate::type_defines::additional_defines::additional_statement::fake_additional_statement::FakeAllNoneAdditionalStatement;
    use crate::type_defines::additional_defines::additional_statement::AdditionalStatement;
    use crate::type_defines::generators::mapper::LangTypeMapper;
    use crate::types::property_type::PropertyType;
    use crate::types::type_name::TypeName;
    use crate::{
        type_defines::generators::mapper::fake_mapper::FakeLangTypeMapper,
        types::property_key::PropertyKey,
    };

    use super::{PropertyStatementGenerator, TypeDefineGenerator, TypeStructureGenerator};
    pub struct FakePropertyStatementGenerator;
    impl PropertyStatementGenerator for FakePropertyStatementGenerator {
        fn generate<M: LangTypeMapper, A: AdditionalStatement>(
            &self,
            type_name: &TypeName,
            property_key: &PropertyKey,
            property_type: &PropertyType,
            mapper: &M,
            a: &A,
        ) -> String {
            let mut result = String::new();
            if let Some(comment) = a.get_property_comment(type_name, property_key) {
                result += &comment;
            };
            if let Some(attribute) = a.get_property_attribute(type_name, property_key) {
                result += &attribute;
            };
            let property_type = if a.is_property_optional(type_name, property_key) {
                mapper.case_optional_type(mapper.case_property_type(property_type))
            } else {
                mapper.case_property_type(property_type)
            };
            format!("{}{}: {},", result, property_key.as_str(), property_type)
        }
    }
    pub struct FakeTypeStructureGenerator;
    impl TypeStructureGenerator for FakeTypeStructureGenerator {
        const TYPE_PREFIX: &'static str = "struct";
        fn generate_case_composite<A: AdditionalStatement>(
            &self,
            type_name: &TypeName,
            properties_statement: String,
            a: &A,
        ) -> String {
            let mut result = String::new();
            if let Some(comment) = a.get_type_comment(type_name) {
                result += &comment;
            };
            if let Some(attribute) = a.get_type_attribute(type_name) {
                result += &attribute;
            };
            format!(
                "{}{} {} {{{}}}",
                result,
                Self::TYPE_PREFIX,
                type_name.as_str(),
                properties_statement
            )
        }
        fn generate_case_primitive<M: LangTypeMapper, A: AdditionalStatement>(
            &self,
            primitive_type: &crate::types::structures::PrimitiveTypeStructure,
            mapper: &M,
            a: &A,
        ) -> String {
            let mut result = String::new();
            if let Some(comment) = a.get_type_comment(&primitive_type.name) {
                result += &comment;
            };
            if let Some(attribute) = a.get_type_attribute(&primitive_type.name) {
                result += &attribute;
            };
            format!(
                "{}type {} = {};",
                result,
                primitive_type.name.as_str(),
                mapper.case_primitive(&primitive_type.primitive_type)
            )
        }
    }
    #[cfg(test)]
    impl
        TypeDefineGenerator<
            FakeTypeStructureGenerator,
            FakePropertyStatementGenerator,
            FakeLangTypeMapper,
            FakeAllNoneAdditionalStatement,
        >
    {
        pub fn new_none_additional_fake() -> Self {
            let mapper = FakeLangTypeMapper;
            Self {
                mapper,
                type_statement_generator: FakeTypeStructureGenerator,
                property_statement_generator: FakePropertyStatementGenerator,
                additional_statement: FakeAllNoneAdditionalStatement,
            }
        }
    }
}
#[cfg(test)]

mod test_type_define_statement_generator {

    use crate::{
        type_defines::generators::generator::TypeDefineGenerator,
        types::{
            primitive_type::primitive_type_factories::*, property_type::property_type_factories::*,
            structures::TypeStructure,
        },
    };
    #[test]
    fn test_case_primitive() {
        let simple_statement = TypeStructure::make_primitive("Test", make_string());
        let tobe = "type Test = String;".to_string();
        let generator = TypeDefineGenerator::new_none_additional_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_optional_case() {
        let simple_statement = TypeStructure::make_composite(
            "Test",
            vec![
                ("id", make_optional_type(make_primitive_type(make_usize()))),
                (
                    "child",
                    make_array_type(make_optional_type(make_custom_type("Child"))),
                ),
            ],
        );
        let tobe = "struct Test {child: Vec<Option<Child>>,id: Option<usize>,}".to_string();
        let generator = TypeDefineGenerator::new_none_additional_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_nest_array_case() {
        let simple_statement = TypeStructure::make_composite(
            "Test",
            vec![
                ("id", make_primitive_type(make_usize())),
                (
                    "child",
                    make_array_type(make_array_type(make_custom_type("Child"))),
                ),
            ],
        );
        let tobe = "struct Test {child: Vec<Vec<Child>>,id: usize,}".to_string();
        let generator = TypeDefineGenerator::new_none_additional_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_array_case() {
        let simple_statement = TypeStructure::make_composite(
            "Test",
            vec![
                ("id", make_primitive_type(make_usize())),
                ("child", make_array_type(make_custom_type("Child"))),
            ],
        );
        let tobe = "struct Test {child: Vec<Child>,id: usize,}".to_string();
        let generator = TypeDefineGenerator::new_none_additional_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_has_child_case() {
        let simple_statement = TypeStructure::make_composite(
            "Test",
            vec![
                ("id", make_primitive_type(make_usize())),
                ("child", make_custom_type("Child")),
            ],
        );
        let tobe = "struct Test {child: Child,id: usize,}".to_string();
        let generator = TypeDefineGenerator::new_none_additional_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_simple_case() {
        let simple_statement =
            TypeStructure::make_composite("Test", vec![("id", make_primitive_type(make_usize()))]);
        let tobe = "struct Test {id: usize,}".to_string();
        let generator = TypeDefineGenerator::new_none_additional_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
        let simple_statement = TypeStructure::make_composite(
            "Test",
            vec![
                ("id", make_primitive_type(make_usize())),
                ("name", make_primitive_type(make_string())),
            ],
        );
        let tobe = "struct Test {id: usize,name: String,}".to_string();
        let generator = TypeDefineGenerator::new_none_additional_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
}
