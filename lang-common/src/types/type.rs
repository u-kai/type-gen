use std::collections::HashMap;

use npc::convertor::NamingPrincipalConvertor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    name: TypeName,
    kind: TypeKind,
}
impl Type {
    pub fn new(name: impl Into<TypeName>, kind: TypeKind) -> Self {
        Self {
            name: name.into(),
            kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    Any,
    ChildType(Box<Type>),
    Primitive(PrimitiveType),
    Composite(CompositeType),
    Optional(Box<TypeKind>),
    Array(Box<TypeKind>),
}
impl TypeKind {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositeType {
    properties: HashMap<PropertyKey, TypeKind>,
}
impl CompositeType {
    pub fn new(properties: HashMap<PropertyKey, TypeKind>) -> Self {
        Self { properties }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeName(String);
impl TypeName {
    pub fn new(str: String) -> Self {
        Self(str)
    }
    fn spawn_child(&self, child: &PropertyKey) -> Self {
        let child = NamingPrincipalConvertor::new(&child.0).to_pascal();
        TypeName(format!("{}{}", self.0, child))
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyKey(String);
impl PropertyKey {
    pub(crate) fn to_type_name(&self, parent_type_name: &TypeName) -> TypeName {
        TypeName(format!(
            "{}{}",
            parent_type_name.0,
            NamingPrincipalConvertor::new(&self.0).to_pascal()
        ))
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
pub(crate) mod fakes {
    use super::*;
    use std::collections::HashMap;

    pub fn make_array_type_easy(content: TypeKind) -> TypeKind {
        TypeKind::Array(Box::new(content))
    }
    pub fn make_child_type(child_type: Type) -> TypeKind {
        TypeKind::ChildType(Box::new(child_type))
    }
    pub fn make_type_easy(name: impl Into<TypeName>, content: TypeKind) -> Type {
        Type {
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
