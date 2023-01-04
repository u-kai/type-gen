use crate::types::{property_key::PropertyKey, type_name::TypeName};

use super::{
    additional_defines::{
        additional_statement::AdditionalStatement, attribute_store::Attribute,
        comment_store::Comment, visibility_store::Visibility,
    },
    generators::{
        generator::{PropertyStatementGenerator, TypeDefineGenerator, TypeStatementGenerator},
        mapper::LangTypeMapper,
    },
};

pub trait TypeDefineBuilder<T, P, M, A, V, C, At>
where
    T: TypeStatementGenerator<M>, //, A>,
    P: PropertyStatementGenerator<M, A>,
    M: LangTypeMapper,
    A: AdditionalStatement,
    V: Visibility,
    C: Comment,
    At: Attribute,
{
    fn build(self) -> TypeDefineGenerator<T, P, M, A>;
    fn set_all_type_optional(self, is_all_optioal: bool) -> Self;
    fn set_all_type_visibility(self, visibility: V) -> Self;
    fn set_all_property_visibility(self, visibility: V) -> Self;
    fn set_all_type_comment(self, comment: C) -> Self;
    fn set_all_property_comment(self, comment: C) -> Self;
    fn set_all_type_attribute(self, attribute: At) -> Self;
    fn set_all_property_attribute(self, attribute: At) -> Self;
    fn add_type_attribute(self, type_name: impl Into<TypeName>, attribute: At) -> Self;
    fn add_property_attribute(
        self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        attribute: At,
    ) -> Self;
    fn add_type_comment(self, type_name: impl Into<TypeName>, comment: C) -> Self;
    fn add_property_comment(
        self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        comment: C,
    ) -> Self;
    fn add_optional(
        self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
    ) -> Self;
    fn add_require(
        self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
    ) -> Self;
    fn add_type_visibility(self, type_name: impl Into<TypeName>, visibility: V) -> Self;
    fn add_property_visibility(
        self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        visibility: V,
    ) -> Self;
}
