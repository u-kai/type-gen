use crate::{
    type_defines::type_define::TypeDefine,
    types::{
        property_key::PropertyKey,
        property_type::PropertyType,
        structures::{AliasTypeStructure, CompositeTypeStructure, TypeStructure},
        type_name::TypeName,
    },
};

use super::mapper::LangTypeMapper;

pub struct TypeDefineGenerator<T, P, M>
where
    //T: TypeStatementGenerator,
    T: TypeStatementGenerator<Mapper = M>,
    P: PropertyStatementGenerator<M>,
    M: LangTypeMapper,
{
    type_statement_generator: T,
    property_statement_generator: P,
    mapper: M,
}
impl<T, P, M> TypeDefineGenerator<T, P, M>
where
    T: TypeStatementGenerator<Mapper = M>,
    //T: TypeStatementGenerator,
    P: PropertyStatementGenerator<M>,
    M: LangTypeMapper,
{
    pub fn new(type_statement_generator: T, property_statement_generator: P, mapper: M) -> Self {
        Self {
            type_statement_generator,
            property_statement_generator,
            mapper,
        }
    }
    pub fn generate_concat_define(&self, structures: Vec<TypeStructure>) -> TypeDefine {
        self.generate(structures)
            .into_iter()
            .reduce(|acc, cur| format!("{}\n{}\n", acc, cur))
            .unwrap()
    }
    pub fn generate(&self, structures: Vec<TypeStructure>) -> Vec<TypeDefine> {
        structures
            .into_iter()
            .map(|s| self.generate_one(s))
            .collect()
    }
    pub fn generate_one(&self, structure: TypeStructure) -> TypeDefine {
        match structure {
            TypeStructure::Composite(composite) => {
                let properties_statement =
                    composite
                        .iter()
                        .fold(String::new(), |acc, (property_key, property_type)| {
                            let property_statement = self.property_statement_generator.generate(
                                &composite.type_name(),
                                property_key,
                                property_type,
                                &self.mapper,
                            );
                            format!("{}{}", acc, property_statement)
                        });
                self.type_statement_generator
                    .generate_case_composite(&composite, properties_statement)
            }
            TypeStructure::Alias(primitive) => self
                .type_statement_generator
                .generate_case_alias(&primitive, &self.mapper),
        }
    }
}

pub trait TypeStatementGenerator {
    const TYPE_PREFIX: &'static str = "class";
    type Mapper: LangTypeMapper;
    fn generate_case_composite(
        &self,
        composite_type: &CompositeTypeStructure,
        properties_statement: String,
    ) -> String;

    fn generate_case_alias(&self, alias_type: &AliasTypeStructure, mapper: &Self::Mapper)
        -> String;
    //    fn generate_case_alias(&self, primitive_type: &AliasTypeStructure, mapper: &M) -> String;
}
pub trait PropertyStatementGenerator<M>
where
    M: LangTypeMapper,
{
    fn generate(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
        property_type: &PropertyType,
        mapper: &M,
    ) -> String;
}
#[cfg(test)]
pub mod fakes {
    use crate::type_defines::generators::mapper::LangTypeMapper;
    use crate::types::property_type::PropertyType;
    use crate::types::structures::CompositeTypeStructure;
    use crate::types::type_name::TypeName;
    use crate::{
        type_defines::generators::mapper::fake_mapper::FakeLangTypeMapper,
        types::property_key::PropertyKey,
    };

    use super::{PropertyStatementGenerator, TypeDefineGenerator, TypeStatementGenerator};
    pub struct FakePropertyStatementGenerator;
    impl<M> PropertyStatementGenerator<M> for FakePropertyStatementGenerator
    where
        M: LangTypeMapper,
    {
        fn generate(
            &self,
            _: &TypeName,
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
    pub struct FakeTypeStatementGenerator;
    impl TypeStatementGenerator for FakeTypeStatementGenerator {
        type Mapper = FakeLangTypeMapper;
        const TYPE_PREFIX: &'static str = "struct";
        fn generate_case_composite(
            &self,
            compsite_type: &CompositeTypeStructure,
            properties_statement: String,
        ) -> String {
            format!(
                "struct {} {{{}}}",
                compsite_type.type_name().as_str(),
                properties_statement
            )
        }
        //fn generate_case_alias<M: LangTypeMapper>(
        fn generate_case_alias(
            &self,
            primitive_type: &crate::types::structures::AliasTypeStructure,
            mapper: &Self::Mapper,
            //mapper: &M,
        ) -> String {
            format!(
                "type {} = {};",
                primitive_type.name.as_str(),
                mapper.case_property_type(&primitive_type.property_type)
            )
        }
    }
    #[cfg(test)]
    impl
        TypeDefineGenerator<
            FakeTypeStatementGenerator,
            FakePropertyStatementGenerator,
            FakeLangTypeMapper,
        >
    {
        pub fn fake_new() -> Self {
            let mapper = FakeLangTypeMapper;
            Self {
                mapper,
                type_statement_generator: FakeTypeStatementGenerator,
                property_statement_generator: FakePropertyStatementGenerator,
            }
        }
    }
}
#[cfg(test)]

mod test_type_define_statement_generator {

    use crate::{
        type_defines::generators::type_define_generator::TypeDefineGenerator,
        types::{
            primitive_type::primitive_type_factories::*, property_type::property_type_factories::*,
            structures::TypeStructure,
        },
    };
    #[test]
    fn test_case_primitive() {
        let simple_statement =
            TypeStructure::make_alias("Test", make_primitive_type(make_string()));
        let tobe = "type Test = String;".to_string();
        let generator = TypeDefineGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
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
        let generator = TypeDefineGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
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
        let generator = TypeDefineGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
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
        let generator = TypeDefineGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
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
        let generator = TypeDefineGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_simple_case() {
        let simple_statement =
            TypeStructure::make_composite("Test", vec![("id", make_primitive_type(make_usize()))]);
        let tobe = "struct Test {id: usize,}".to_string();
        let generator = TypeDefineGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
        assert_eq!(statements, tobe);
        let simple_statement = TypeStructure::make_composite(
            "Test",
            vec![
                ("id", make_primitive_type(make_usize())),
                ("name", make_primitive_type(make_string())),
            ],
        );
        let tobe = "struct Test {id: usize,name: String,}".to_string();
        let generator = TypeDefineGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
        assert_eq!(statements, tobe);
    }
}
