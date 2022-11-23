use crate::type_defines::type_define::{LangAttribute, LangComment, LangVisibility, TypeDefine};

use super::mapper::LangTypeMapper;

type TypeDefineStatement = String;
pub trait TypeDefineStatementGenerator<V, C, A, M>
where
    V: LangVisibility,
    C: LangComment,
    A: LangAttribute,
    M: LangTypeMapper,
{
    fn generate(&self, type_define: TypeDefine<V, C, A>, mapper: M) -> Vec<TypeDefineStatement>;
}

#[cfg(test)]
pub mod fakes {
    use crate::type_defines::generators::mapper::{LangTypeMapper, TypeStatement};
    use crate::type_defines::type_define::{
        LangAttribute, LangComment, LangVisibility, TypeDefine,
    };

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
        fn case_any(&self) -> TypeStatement {
            String::from("any")
        }
        fn case_boolean(&self) -> TypeStatement {
            String::from("bool")
        }
        fn case_float(&self) -> TypeStatement {
            String::from("f64")
        }
        fn case_isize(&self) -> TypeStatement {
            String::from("isize")
        }
        fn case_usize(&self) -> TypeStatement {
            String::from("usize")
        }
        fn case_null(&self) -> TypeStatement {
            String::from("null")
        }
        fn case_optional_type<T: Into<TypeStatement>>(&self, type_statement: T) -> TypeStatement {
            format!("Option<{}>", type_statement.into())
        }
        fn case_string(&self) -> TypeStatement {
            String::from("String")
        }
        fn case_array_type<T: Into<TypeStatement>>(&self, type_statement: T) -> TypeStatement {
            format!("Vec<{}>", type_statement.into())
        }
    }
    pub struct FakeTypeGenerator;

    impl<V, C, A, M> TypeDefineStatementGenerator<V, C, A, M> for FakeTypeGenerator
    where
        V: LangVisibility,
        C: LangComment,
        A: LangAttribute,
        M: LangTypeMapper,
    {
        fn generate(
            &self,
            type_define: TypeDefine<V, C, A>,
            mapper: M,
        ) -> Vec<TypeDefineStatement> {
            vec!["struct Test {id: usize,}".to_string()]
        }
    }
}
#[cfg(test)]

mod test_type_define_statement_generator {

    use super::{
        fakes::{
            FakeLangAttribute, FakeLangComment, FakeLangTypeMapper, FakeLangVisibility,
            FakeTypeGenerator,
        },
        *,
    };
    use crate::types::structure::{fakes::*, *};
    impl TypeDefine<FakeLangVisibility, FakeLangComment, FakeLangAttribute> {
        fn new_simple(type_: TypeStructure, visi: &str) -> Self {
            TypeDefine::new(
                type_,
                FakeLangVisibility::new(visi),
                Some(FakeLangComment::new(vec![""])),
                Some(FakeLangAttribute::new("")),
            )
        }
    }
    #[test]
    fn test_simple_case() {
        let simple_define = TypeDefine::new_simple(
            make_type_easy(
                "Test",
                make_composite_type_easy(vec![("id", type_kind_usize())]),
            ),
            "",
        );
        let tobe = vec!["struct Test {id: usize,}".to_string()];
        let generator = FakeTypeGenerator;
        assert_eq!(generator.generate(simple_define, FakeLangTypeMapper), tobe);
        let simple_define = TypeDefine::new_simple(
            make_type_easy(
                "Test",
                make_composite_type_easy(vec![
                    ("id", type_kind_usize()),
                    ("name", type_kind_string()),
                ]),
            ),
            "",
        );
        let tobe = vec!["struct Test {id: usize,name:String,}".to_string()];
        let generator = FakeTypeGenerator;
        assert_eq!(generator.generate(simple_define, FakeLangTypeMapper), tobe);
    }
}
