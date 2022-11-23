use crate::{
    type_defines::type_define::TypeDefine,
    types::{
        property_key::PropertyKey,
        property_type::PropertyType,
        structures::{PrimitiveTypeStructure, TypeStructure},
        type_name::TypeName,
    },
};

use super::mapper::LangTypeMapper;

pub struct TypeDefineGenerator<T, P, M>
where
    T: TypeStructureGenerator,
    P: PropertyStatementGenerator,
    M: LangTypeMapper,
{
    type_statement_generator: T,
    property_statement_generator: P,
    mapper: M,
}
impl<T, P, M> TypeDefineGenerator<T, P, M>
where
    T: TypeStructureGenerator,
    P: PropertyStatementGenerator,
    M: LangTypeMapper,
{
    pub fn new(type_statement_generator: T, property_statement_generator: P, mapper: M) -> Self {
        Self {
            type_statement_generator,
            property_statement_generator,
            mapper,
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
                                self.property_statement_generator
                                    .generate(k, v, &self.mapper)
                            )
                        });
                self.type_statement_generator
                    .generate_case_composite(&composite.name, properties_statement)
            }
            TypeStructure::Primitive(primitive) => self
                .type_statement_generator
                .generate_case_primitive(&primitive, &self.mapper),
        }
    }
}

pub trait TypeStructureGenerator {
    const TYPE_PREFIX: &'static str = "class";
    fn generate_case_composite(&self, type_name: &TypeName, properties_statement: String)
        -> String;
    fn generate_case_primitive<M: LangTypeMapper>(
        &self,
        primitive_type: &PrimitiveTypeStructure,
        mapper: &M,
    ) -> String;
}
pub trait PropertyStatementGenerator {
    fn generate<M: LangTypeMapper>(
        &self,
        property_key: &PropertyKey,
        property_type: &PropertyType,
        mapper: &M,
    ) -> String;
}
#[cfg(test)]
pub mod fakes {
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
        fn generate<M: LangTypeMapper>(
            &self,
            property_key: &PropertyKey,
            property_type: &PropertyType,
            mapper: &M,
        ) -> String {
            format!(
                "{}: {},",
                property_key.as_str(),
                mapper.case_property_type(property_type)
            )
        }
    }
    pub struct FakeTypeStructureGenerator;
    impl TypeStructureGenerator for FakeTypeStructureGenerator {
        const TYPE_PREFIX: &'static str = "struct";
        fn generate_case_composite(
            &self,
            type_name: &TypeName,
            properties_statement: String,
        ) -> String {
            format!(
                "{} {} {{{}}}",
                Self::TYPE_PREFIX,
                type_name.as_str(),
                properties_statement
            )
        }
        fn generate_case_primitive<M: LangTypeMapper>(
            &self,
            primitive_type: &crate::types::structures::PrimitiveTypeStructure,
            mapper: &M,
        ) -> String {
            format!(
                "type {} = {};",
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
        >
    {
        pub fn new_fake() -> Self {
            let mapper = FakeLangTypeMapper;
            Self {
                mapper,
                type_statement_generator: FakeTypeStructureGenerator,
                property_statement_generator: FakePropertyStatementGenerator,
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
        let generator = TypeDefineGenerator::new_fake();
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
        let generator = TypeDefineGenerator::new_fake();
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
        let generator = TypeDefineGenerator::new_fake();
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
        let generator = TypeDefineGenerator::new_fake();
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
        let generator = TypeDefineGenerator::new_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_simple_case() {
        let simple_statement =
            TypeStructure::make_composite("Test", vec![("id", make_primitive_type(make_usize()))]);
        let tobe = "struct Test {id: usize,}".to_string();
        let generator = TypeDefineGenerator::new_fake();
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
        let generator = TypeDefineGenerator::new_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
}
