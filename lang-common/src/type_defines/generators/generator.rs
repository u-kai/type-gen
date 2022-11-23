use crate::{
    type_defines::type_define::{LangAttribute, LangComment, LangVisibility},
    types::statement::TypeStatement,
};

use super::mapper::LangTypeMapper;

type TypeDefineStatement = String;
pub trait TypeDefineStatementGenerator<V, C, A, M>
where
    V: LangVisibility,
    C: LangComment,
    A: LangAttribute,
    M: LangTypeMapper,
{
    fn new(visibility: V, comment: C, attribute: A, mapper: M) -> Self;
    fn generate(&self, statement: TypeStatement) -> TypeDefineStatement;
}

#[cfg(test)]
pub mod fakes {
    use crate::type_defines::generators::mapper::{LangTypeMapper, TypeString};
    use crate::type_defines::type_define::{LangAttribute, LangComment, LangVisibility};
    use crate::types::statement::TypeStatement;

    use super::{TypeDefineStatement, TypeDefineStatementGenerator};

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
    pub struct FakeTypeGenerator {
        visi: FakeLangVisibility,
        comment: FakeLangComment,
        attr: FakeLangAttribute,
        mapper: FakeLangTypeMapper,
    }
    impl FakeTypeGenerator {
        pub const PREFIX: &'static str = "struct";
        pub fn new_easy() -> Self {
            let visi = FakeLangVisibility::new("");
            let comment = FakeLangComment::new(vec![""]);
            let attr = FakeLangAttribute::new("");
            let mapper = FakeLangTypeMapper;
            Self {
                visi,
                comment,
                attr,
                mapper,
            }
        }
    }
    impl
        TypeDefineStatementGenerator<
            FakeLangVisibility,
            FakeLangComment,
            FakeLangAttribute,
            FakeLangTypeMapper,
        > for FakeTypeGenerator
    {
        fn new(
            visi: FakeLangVisibility,
            comment: FakeLangComment,
            attr: FakeLangAttribute,
            mapper: FakeLangTypeMapper,
        ) -> Self {
            Self {
                visi,
                comment,
                attr,
                mapper,
            }
        }
        fn generate(&self, statement: TypeStatement) -> TypeDefineStatement {
            match statement {
                TypeStatement::Composite(composite) => {
                    let properties_statement =
                        composite
                            .properties
                            .iter()
                            .fold(String::new(), |acc, (k, v)| {
                                format!(
                                    "{}{}: {},",
                                    acc,
                                    k.as_str(),
                                    self.mapper.case_property_type(v)
                                )
                            });
                    format!(
                        "{} {} {{{}}}",
                        Self::PREFIX,
                        composite.name.as_str(),
                        properties_statement
                    )
                }
                TypeStatement::Primitive(primitive) => {
                    format!(
                        "type {} = {};",
                        primitive.name.as_str(),
                        self.mapper.case_primitive(&primitive.primitive_type)
                    )
                }
            }
        }
    }
}
#[cfg(test)]

mod test_type_define_statement_generator {

    use super::{fakes::FakeTypeGenerator, *};
    use crate::types::{
        statement::{
            property_type_factories::{
                make_array_type, make_custom_type, make_optional_type, make_primitive_type,
            },
            TypeStatement,
        },
        structures::primitive_type_factories::*,
    };
    #[test]
    fn test_case_primitive() {
        let simple_statement = TypeStatement::make_primitive("Test", make_string());
        let tobe = "type Test = String;".to_string();
        let generator = FakeTypeGenerator::new_easy();
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
        let generator = FakeTypeGenerator::new_easy();
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
        let generator = FakeTypeGenerator::new_easy();
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
        let generator = FakeTypeGenerator::new_easy();
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
        let generator = FakeTypeGenerator::new_easy();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_simple_case() {
        let simple_statement =
            TypeStatement::make_composite("Test", vec![("id", make_primitive_type(make_usize()))]);
        let tobe = "struct Test {id: usize,}".to_string();
        let generator = FakeTypeGenerator::new_easy();
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
        let generator = FakeTypeGenerator::new_easy();
        let statements = generator.generate(simple_statement);
        assert_eq!(statements, tobe);
    }
}
