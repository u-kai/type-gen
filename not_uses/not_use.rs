use std::collections::HashMap;

use npc::convertor::NamingPrincipalConvertor;

use super::statement::TypeStatement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeStructure {
    name: TypeName,
    kind: TypeKind,
}
impl TypeStructure {
    pub fn new(name: impl Into<TypeName>, kind: TypeKind) -> Self {
        Self {
            name: name.into(),
            kind,
        }
    }
    pub fn into_statements(self) -> Vec<TypeStatement> {
        println!("start split {:#?}", self);
        if !self.has_children() {
            //    return vec![self];
            return vec![];
        }
        match self.kind {
            TypeKind::ChildType(child) => child.into_statements(),
            TypeKind::Composite(composite) => {
                fn case_composite(composite: CompositeType) -> Vec<TypeStatement> {
                    let mut result = Vec::new();
                    for (_, composite) in composite.properties {
                        match composite {
                            TypeKind::ChildType(child) => {
                                let mut children = child.into_statements();
                                result.append(&mut children);
                            }

                            TypeKind::Composite(composite) => {
                                let mut children = case_composite(composite);
                                result.append(&mut children);
                            }

                            TypeKind::Array(array) => {
                                todo!()
                            }
                            _ => (),
                        }
                    }
                    result
                }
                case_composite(composite)
            }
            TypeKind::Array(type_kind) => todo!(),
            _ => vec![],
        }
    }
    fn has_children(&self) -> bool {
        self.kind.has_children()
    }
}

#[cfg(test)]
mod test_split_type {
    use crate::types::{
        not_use::fakes::{
            make_child_type, make_composite_type_easy, make_type_easy, type_kind_string,
            type_kind_usize,
        },
        statement::CompositeTypeStatement,
    };

    use super::*;

    //#[test]
    //fn test_nest_composite_type_case() {
    //// ```
    //// // primitive_type is
    //// struct Test {
    ////     id: usize,
    ////     name: String,
    ////     child: Child,
    //// }
    //// struct Child {
    ////     id: usize,
    ////     data: ChildData,
    //// }
    //// struct ChildData {
    ////     name: String,
    //// }
    //// ```
    //let composite = make_composite_type_easy(vec![
    //("id", type_kind_usize()),
    //("name", type_kind_string()),
    //(
    //"child",
    //make_child_type(make_type_easy(
    //"Child",
    //make_composite_type_easy(vec![
    //("id", type_kind_usize()),
    //(
    //"data",
    //make_child_type(make_type_easy(
    //"ChildData",
    //make_composite_type_easy(vec![("name", type_kind_string())]),
    //)),
    //),
    //]),
    //)),
    //),
    //]);
    //let has_children_type = make_type_easy("Test", composite);
    //let tobe = vec![];

    //assert_eq!(has_children_type.split(), tobe);
    //}
    //#[test]
    //fn test_simple_composite_type_case() {
    //let composite = make_composite_type_easy(vec![
    //("name", type_kind_string()),
    //("id", type_kind_usize()),
    //]);
    //let primitive_type = make_type_easy("Test", composite.clone());
    //let tobe = vec![TypeStatement::Composite(CompositeTypeStatement::new_easy(
    //"Test",
    //vec!["name"],
    //))];
    //assert_eq!(primitive_type.into_statements(), tobe);
    //}
    //#[test]
    //fn test_primitive_type_case() {
    //let primitive_type = make_type_easy("name", type_kind_string());
    //assert_eq!(primitive_type.clone().split(), vec![primitive_type]);
    //}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    Any,
    ChildType(Box<TypeStructure>),
    Primitive(PrimitiveType),
    Composite(CompositeType),
    Array(Box<TypeKind>),
}
impl TypeKind {
    fn has_children(&self) -> bool {
        match self {
            Self::ChildType(_) => true,
            Self::Composite(composite) => {
                composite.properties.iter().any(|(_, v)| v.has_children())
            }
            Self::Array(type_kind) => type_kind.has_children(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod test_type_kind {
    use crate::types::not_use::fakes::{make_child_type, make_type_easy};

    use super::fakes::{make_composite_type_easy, type_kind_string, type_kind_usize};
    #[test]
    fn test_has_children() {
        let has_not_child = make_composite_type_easy(vec![
            ("id", type_kind_usize()),
            ("name", type_kind_string()),
        ]);
        assert!(!has_not_child.has_children());
        let has_children = make_composite_type_easy(vec![
            ("id", type_kind_usize()),
            ("name", type_kind_string()),
            (
                "child",
                make_child_type(make_type_easy(
                    "Child",
                    make_composite_type_easy(vec![(
                        "data",
                        make_child_type(make_type_easy(
                            "ChildData",
                            make_composite_type_easy(vec![("id", type_kind_usize())]),
                        )),
                    )]),
                )),
            ),
        ]);
        assert!(has_children.has_children());
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositeType {
    properties: HashMap<PropertyKey, TypeKind>,
}
impl CompositeType {
    pub fn new(properties: HashMap<PropertyKey, TypeKind>) -> Self {
        Self { properties }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeName(String);
impl TypeName {
    pub fn new(str: String) -> Self {
        Self(str)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<I> From<I> for TypeName
where
    I: Into<String>,
{
    fn from(str: I) -> Self {
        let str: String = str.into();
        TypeName::new(str)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PropertyKey(String);
impl PropertyKey {
    pub(crate) fn to_type_name(&self, parent_type_name: &TypeName) -> TypeName {
        TypeName(format!(
            "{}{}",
            parent_type_name.0,
            NamingPrincipalConvertor::new(&self.0).to_pascal()
        ))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<I> From<I> for PropertyKey
where
    I: Into<String>,
{
    fn from(source: I) -> Self {
        Self(source.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    String,
    Boolean,
    Number(Number),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Number {
    Usize,
    Isize,
    Float,
}

#[cfg(test)]
pub(crate) mod primitive_type_factories {
    use super::{Number, PrimitiveType};
    pub fn make_string() -> PrimitiveType {
        PrimitiveType::String
    }
    pub fn make_bool() -> PrimitiveType {
        PrimitiveType::Boolean
    }
    pub fn make_usize() -> PrimitiveType {
        PrimitiveType::Number(Number::Usize)
    }
    pub fn make_isize() -> PrimitiveType {
        PrimitiveType::Number(Number::Isize)
    }
    pub fn make_float() -> PrimitiveType {
        PrimitiveType::Number(Number::Float)
    }
}

#[cfg(test)]
pub(crate) mod fakes {
    use super::*;
    use std::collections::HashMap;

    pub fn make_array_type_easy(content: TypeKind) -> TypeKind {
        TypeKind::Array(Box::new(content))
    }
    pub fn make_child_type(child_type: TypeStructure) -> TypeKind {
        TypeKind::ChildType(Box::new(child_type))
    }
    pub fn make_type_easy(name: impl Into<TypeName>, content: TypeKind) -> TypeStructure {
        TypeStructure {
            name: name.into(),
            kind: content,
        }
    }
    pub fn make_composite_type_easy(source: Vec<(&str, TypeKind)>) -> TypeKind {
        let properties = source
            .into_iter()
            .map(|(key, type_kind)| (PropertyKey::from(key), type_kind))
            .collect::<HashMap<_, _>>();
        TypeKind::Composite(CompositeType { properties })
    }
    pub fn type_kind_string() -> TypeKind {
        TypeKind::Primitive(PrimitiveType::String)
    }
    pub fn type_kind_usize() -> TypeKind {
        TypeKind::Primitive(PrimitiveType::Number(Number::Usize))
    }
    pub fn type_kind_isize() -> TypeKind {
        TypeKind::Primitive(PrimitiveType::Number(Number::Isize))
    }
    pub fn type_kind_float() -> TypeKind {
        TypeKind::Primitive(PrimitiveType::Number(Number::Float))
    }
}
