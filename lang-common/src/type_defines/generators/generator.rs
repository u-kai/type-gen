use crate::types::{
    statement::{PrimitiveTypeStatement, PropertyType, TypeStatement},
    structures::{PropertyKey, TypeName},
};

use super::mapper::LangTypeMapper;

type TypeDefineStatement = String;
pub struct TypeDefineStatementGenerator<T, P, M>
where
    T: TypeStatementGenerator,
    P: PropertyStatementGenerator,
    M: LangTypeMapper,
{
    type_statement_generator: T,
    property_statement_generator: P,
    mapper: M,
}
impl<T, P, M> TypeDefineStatementGenerator<T, P, M>
where
    T: TypeStatementGenerator,
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
    pub fn generate(&self, statement: TypeStatement) -> TypeDefineStatement {
        match statement {
            TypeStatement::Composite(composite) => {
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
            TypeStatement::Primitive(primitive) => self
                .type_statement_generator
                .generate_case_primitive(&primitive, &self.mapper),
        }
    }
}

pub trait TypeStatementGenerator {
    const TYPE_PREFIX: &'static str = "class";
    fn generate_case_composite(&self, type_name: &TypeName, properties_statement: String)
        -> String;
    fn generate_case_primitive<M: LangTypeMapper>(
        &self,
        primitive_type: &PrimitiveTypeStatement,
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
    use crate::type_defines::generators::mapper::{LangTypeMapper, TypeString};
    use crate::type_defines::type_define::{LangAttribute, LangComment, LangVisibility};
    use crate::types::statement::PropertyType;
    use crate::types::structures::{PropertyKey, TypeName};

    use super::{PropertyStatementGenerator, TypeDefineStatementGenerator, TypeStatementGenerator};
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
    pub struct FakeTypeStatementGenerator;
    impl TypeStatementGenerator for FakeTypeStatementGenerator {
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
            primitive_type: &crate::types::statement::PrimitiveTypeStatement,
            mapper: &M,
        ) -> String {
            format!(
                "type {} = {};",
                primitive_type.name.as_str(),
                mapper.case_primitive(&primitive_type.primitive_type)
            )
        }
    }

    pub struct FakeLangVisibility {
        all_visibility: String,
    }
    impl FakeLangVisibility {
        pub fn new(all_visibility: impl Into<String>) -> Self {
            Self {
                all_visibility: all_visibility.into(),
            }
        }
    }
    impl LangVisibility for FakeLangVisibility {
        fn to_define(self) -> String {
            self.all_visibility
        }
    }
    pub struct FakeLangComment {
        comments: Vec<String>,
    }
    impl FakeLangComment {
        pub fn new(comments: Vec<impl Into<String>>) -> Self {
            Self {
                comments: comments.into_iter().map(|s| s.into()).collect(),
            }
        }
    }
    impl LangComment for FakeLangComment {
        fn to_define(self) -> String {
            self.comments
                .into_iter()
                .fold(String::new(), |mut acc, cur| {
                    acc = format!("{}//{}\n", acc, cur);
                    acc
                })
        }
    }
    pub struct FakeLangAttribute {
        attr: String,
    }
    impl FakeLangAttribute {
        pub fn new(attr: impl Into<String>) -> Self {
            Self { attr: attr.into() }
        }
    }
    impl LangAttribute for FakeLangAttribute {
        fn to_define(self) -> String {
            self.attr
        }
    }
    pub struct FakeLangTypeMapper;
    impl LangTypeMapper for FakeLangTypeMapper {
        fn case_any(&self) -> TypeString {
            String::from("any")
        }
        fn case_boolean(&self) -> TypeString {
            String::from("bool")
        }
        fn case_float(&self) -> TypeString {
            String::from("f64")
        }
        fn case_isize(&self) -> TypeString {
            String::from("isize")
        }
        fn case_usize(&self) -> TypeString {
            String::from("usize")
        }
        fn case_null(&self) -> TypeString {
            String::from("null")
        }
        fn case_optional_type<T: Into<TypeString>>(&self, type_statement: T) -> TypeString {
            format!("Option<{}>", type_statement.into())
        }
        fn case_string(&self) -> TypeString {
            String::from("String")
        }
        fn case_array_type<T: Into<TypeString>>(&self, type_statement: T) -> TypeString {
            format!("Vec<{}>", type_statement.into())
        }
    }
    #[cfg(test)]
    impl
        TypeDefineStatementGenerator<
            FakeTypeStatementGenerator,
            FakePropertyStatementGenerator,
            FakeLangTypeMapper,
        >
    {
        pub fn new_fake() -> Self {
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

    use super::fakes::*;
    use crate::{
        type_defines::generators::generator::TypeDefineStatementGenerator,
        types::{
            statement::{
                property_type_factories::{
                    make_array_type, make_custom_type, make_optional_type, make_primitive_type,
                },
                TypeStatement,
            },
            structures::primitive_type_factories::*,
        },
    };
    #[test]
    fn test_case_primitive() {
        let simple_statement = TypeStatement::make_primitive("Test", make_string());
        let tobe = "type Test = String;".to_string();
        let generator = TypeDefineStatementGenerator::new_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_optional_case() {
        let simple_statement = TypeStatement::make_composite(
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
        let generator = TypeDefineStatementGenerator::new_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_nest_array_case() {
        let simple_statement = TypeStatement::make_composite(
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
        let generator = TypeDefineStatementGenerator::new_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_array_case() {
        let simple_statement = TypeStatement::make_composite(
            "Test",
            vec![
                ("id", make_primitive_type(make_usize())),
                ("child", make_array_type(make_custom_type("Child"))),
            ],
        );
        let tobe = "struct Test {child: Vec<Child>,id: usize,}".to_string();
        let generator = TypeDefineStatementGenerator::new_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_has_child_case() {
        let simple_statement = TypeStatement::make_composite(
            "Test",
            vec![
                ("id", make_primitive_type(make_usize())),
                ("child", make_custom_type("Child")),
            ],
        );
        let tobe = "struct Test {child: Child,id: usize,}".to_string();
        let generator = TypeDefineStatementGenerator::new_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_simple_case() {
        let simple_statement =
            TypeStatement::make_composite("Test", vec![("id", make_primitive_type(make_usize()))]);
        let tobe = "struct Test {id: usize,}".to_string();
        let generator = TypeDefineStatementGenerator::new_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
        let simple_statement = TypeStatement::make_composite(
            "Test",
            vec![
                ("id", make_primitive_type(make_usize())),
                ("name", make_primitive_type(make_string())),
            ],
        );
        let tobe = "struct Test {id: usize,name: String,}".to_string();
        let generator = TypeDefineStatementGenerator::new_fake();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
}
