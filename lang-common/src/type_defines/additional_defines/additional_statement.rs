use crate::types::{property_key::PropertyKey, type_name::TypeName};

use super::{
    attribute_store::{Attribute, AttributeStore},
    comment_store::{Comment, CommentStore},
    optional_type_store::OptionalTypeStore,
    visibility_store::{Visibility, VisibilityStore},
};

pub trait AdditionalStatement {
    fn get_type_comment(&self, type_name: &TypeName) -> Option<String>;
    fn get_property_comment(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> Option<String>;
    fn get_type_visibility(&self, type_name: &TypeName) -> &'static str;
    fn get_property_visibility(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> &'static str;
    fn get_type_attribute(&self, type_name: &TypeName) -> Option<String>;
    fn get_property_attribute(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> Option<String>;
    fn is_property_optional(&self, type_name: &TypeName, property_key: &PropertyKey) -> bool;
}

#[derive(Debug, Clone)]
pub struct AdditionalStatementProvider<V, C, A>
where
    V: Visibility,
    C: Comment,
    A: Attribute,
{
    attribute_store: AttributeStore<A>,
    comment_store: CommentStore<C>,
    optional_type_store: OptionalTypeStore,
    visibility_store: VisibilityStore<V>,
}

impl<V, C, A> AdditionalStatementProvider<V, C, A>
where
    V: Visibility,
    C: Comment,
    A: Attribute,
{
    pub fn new() -> Self {
        Self {
            attribute_store: AttributeStore::new(),
            comment_store: CommentStore::new(),
            optional_type_store: OptionalTypeStore::default(),
            visibility_store: VisibilityStore::new(),
        }
    }
    pub fn with_default_optional(is_optional_default: bool) -> Self {
        Self {
            attribute_store: AttributeStore::new(),
            comment_store: CommentStore::new(),
            optional_type_store: OptionalTypeStore::new(is_optional_default),
            visibility_store: VisibilityStore::new(),
        }
    }
    // attribute
    pub fn set_all_type_attribute(&mut self, attribute: A) {
        self.attribute_store.set_all_type_attribute(attribute)
    }
    pub fn set_all_property_attribute(&mut self, attribute: A) {
        self.attribute_store.set_all_property_attribute(attribute)
    }
    pub fn add_type_attribute(&mut self, type_name: impl Into<TypeName>, attribute: A) {
        self.attribute_store
            .add_type_attribute(type_name, attribute);
    }
    pub fn add_property_attribute(
        &mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        attribute: A,
    ) {
        self.attribute_store
            .add_property_attribute(type_name, property_key, attribute)
    }
    // comment
    pub fn set_all_type_comment(&mut self, comment: C) {
        self.comment_store.set_all_type_comment(comment)
    }
    pub fn set_all_property_comment(&mut self, comment: C) {
        self.comment_store.set_all_property_comment(comment)
    }
    pub fn add_type_comment(&mut self, type_name: impl Into<TypeName>, comment: C) {
        self.comment_store.add_type_comment(type_name, comment)
    }
    pub fn add_property_comment(
        &mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        comment: C,
    ) {
        self.comment_store
            .add_property_comment(type_name, property_key, comment)
    }
    // visibility
    pub fn set_all_type_visibility(&mut self, visibility: V) {
        self.visibility_store.set_all_type_visibility(visibility)
    }
    pub fn set_all_property_visibility(&mut self, visibility: V) {
        self.visibility_store
            .set_all_property_visibility(visibility)
    }
    pub fn add_type_visibility(&mut self, type_name: impl Into<TypeName>, visibility: V) {
        self.visibility_store
            .add_type_visibility(type_name, visibility);
    }
    pub fn add_property_visibility(
        &mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
        visibility: V,
    ) {
        self.visibility_store
            .add_property_visibility(type_name, property_key, visibility);
    }
    // optional
    pub fn set_all_type_optional(&mut self, is_optional_default: bool) {
        self.optional_type_store
            .set_all_is_optional(is_optional_default)
    }
    pub fn add_optional(
        &mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
    ) {
        self.optional_type_store
            .add_optional(type_name, property_key);
    }
    pub fn add_require(
        &mut self,
        type_name: impl Into<TypeName>,
        property_key: impl Into<PropertyKey>,
    ) {
        self.optional_type_store
            .add_require(type_name, property_key);
    }
}
impl<V, C, A> AdditionalStatement for AdditionalStatementProvider<V, C, A>
where
    V: Visibility,
    C: Comment,
    A: Attribute,
{
    fn get_property_visibility(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> &'static str {
        self.visibility_store
            .get_property_visibility(type_name, property_key)
    }
    fn get_type_visibility(&self, type_name: &TypeName) -> &'static str {
        self.visibility_store.get_type_visibility(type_name)
    }
    fn get_property_comment(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> Option<String> {
        self.comment_store
            .get_property_comment(type_name, property_key)
    }
    fn get_type_comment(&self, type_name: &TypeName) -> Option<String> {
        self.comment_store.get_type_comment(type_name)
    }
    fn is_property_optional(&self, type_name: &TypeName, property_key: &PropertyKey) -> bool {
        self.optional_type_store
            .is_optional(type_name, property_key)
    }
    fn get_property_attribute(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> Option<String> {
        self.attribute_store
            .get_property_attribute(type_name, property_key)
    }
    fn get_type_attribute(&self, type_name: &TypeName) -> Option<String> {
        self.attribute_store.get_type_attribute(type_name)
    }
}

#[cfg(test)]
pub mod fake_additional_statement {
    use crate::types::{property_key::PropertyKey, type_name::TypeName};

    use super::AdditionalStatement;

    pub struct FakeAlwaysSomeAdditionalStatement;
    impl AdditionalStatement for FakeAlwaysSomeAdditionalStatement {
        fn get_property_visibility(
            &self,
            _type_name: &TypeName,
            _property_key: &PropertyKey,
        ) -> &'static str {
            "public "
        }
        fn get_type_visibility(&self, _type_name: &TypeName) -> &'static str {
            "public "
        }
        fn get_property_attribute(
            &self,
            _type_name: &crate::types::type_name::TypeName,
            _property_key: &crate::types::property_key::PropertyKey,
        ) -> Option<String> {
            Some("#[get_property_attribute]".to_string())
        }
        fn get_property_comment(
            &self,
            _type_name: &crate::types::type_name::TypeName,
            _property_key: &crate::types::property_key::PropertyKey,
        ) -> Option<String> {
            Some("// get_property_comment".to_string())
        }
        fn get_type_attribute(
            &self,
            _type_name: &crate::types::type_name::TypeName,
        ) -> Option<String> {
            Some("#[get_type_attribute]".to_string())
        }
        fn get_type_comment(
            &self,
            _type_name: &crate::types::type_name::TypeName,
        ) -> Option<String> {
            Some("// get_type_comment".to_string())
        }
        fn is_property_optional(
            &self,
            _type_name: &crate::types::type_name::TypeName,
            _property_key: &crate::types::property_key::PropertyKey,
        ) -> bool {
            true
        }
    }
    pub struct FakeAllNoneAdditionalStatement;
    impl AdditionalStatement for FakeAllNoneAdditionalStatement {
        fn get_property_visibility(
            &self,
            _type_name: &TypeName,
            _property_key: &PropertyKey,
        ) -> &'static str {
            ""
        }
        fn get_type_visibility(&self, _type_name: &TypeName) -> &'static str {
            ""
        }
        fn get_property_attribute(
            &self,
            _type_name: &crate::types::type_name::TypeName,
            _property_key: &crate::types::property_key::PropertyKey,
        ) -> Option<String> {
            None
        }
        fn get_property_comment(
            &self,
            _type_name: &crate::types::type_name::TypeName,
            _property_key: &crate::types::property_key::PropertyKey,
        ) -> Option<String> {
            None
        }
        fn get_type_attribute(
            &self,
            _type_name: &crate::types::type_name::TypeName,
        ) -> Option<String> {
            None
        }
        fn get_type_comment(
            &self,
            _type_name: &crate::types::type_name::TypeName,
        ) -> Option<String> {
            None
        }
        fn is_property_optional(
            &self,
            _type_name: &crate::types::type_name::TypeName,
            _property_key: &crate::types::property_key::PropertyKey,
        ) -> bool {
            false
        }
    }
}
