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
    fn get_type_visibility(&self, type_name: &TypeName) -> Option<&'static str>;
    fn get_property_visibility(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> Option<&'static str>;
    fn get_type_attribute(&self, type_name: &TypeName) -> Option<String>;
    fn get_property_attribute(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> Option<String>;
    fn is_property_optional(&self, type_name: &TypeName, property_key: &PropertyKey) -> bool;
}

pub struct AdditionalStatementProvider<'a, V>
where
    V: Visibility,
{
    attribute_store: AttributeStore<'a>,
    comment_store: CommentStore<'a>,
    optional_type_store: OptionalTypeStore,
    visibility_store: VisibilityStore<'a, V>,
    comment_prefix: &'static str,
}

impl<'a, V> AdditionalStatementProvider<'a, V>
where
    V: Visibility,
{
    pub fn new() -> Self {
        Self {
            attribute_store: AttributeStore::new(),
            comment_store: CommentStore::new(),
            optional_type_store: OptionalTypeStore::default(),
            visibility_store: VisibilityStore::new(),
            comment_prefix: "// ",
        }
    }
    pub fn with_comment_prefix(comment_prefix: &'static str) -> Self {
        Self {
            attribute_store: AttributeStore::new(),
            comment_store: CommentStore::new(),
            optional_type_store: OptionalTypeStore::default(),
            visibility_store: VisibilityStore::new(),
            comment_prefix,
        }
    }
    pub fn add_type_attribute(&mut self, type_name: &'a TypeName, attribute: Attribute) {
        self.attribute_store
            .add_type_attribute(type_name, attribute);
    }
    pub fn add_property_attribute(
        &mut self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
        attribute: Attribute,
    ) {
        self.attribute_store
            .add_property_attribute(type_name, property_key, attribute)
    }
    pub fn add_type_comment(&mut self, type_name: &'a TypeName, comment: Comment) {
        self.comment_store.add_type_comment(type_name, comment)
    }
    pub fn add_property_comment(
        &mut self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
        comment: Comment,
    ) {
        self.comment_store
            .add_property_comment(type_name, property_key, comment)
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
    pub fn add_type_visibility(&mut self, type_name: &'a TypeName, visibility: V) {
        self.visibility_store
            .add_type_visibility(type_name, visibility);
    }
    pub fn add_property_visibility(
        &mut self,
        type_name: &'a TypeName,
        property_key: &'a PropertyKey,
        visibility: V,
    ) {
        self.visibility_store
            .add_property_visibility(type_name, property_key, visibility);
    }
}
impl<'a, V> AdditionalStatement for AdditionalStatementProvider<'a, V>
where
    V: Visibility,
{
    fn get_property_visibility(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> Option<&'static str> {
        self.visibility_store
            .get_property_visibility(type_name, property_key)
    }
    fn get_type_visibility(&self, type_name: &TypeName) -> Option<&'static str> {
        self.visibility_store.get_type_visibility(type_name)
    }
    fn get_property_comment(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
    ) -> Option<String> {
        let comments = self
            .comment_store
            .get_property_comment(type_name, property_key)?;
        Some(comments.iter().fold(String::new(), |acc, cur| {
            format!("{}{}{}\n", acc, self.comment_prefix, cur)
        }))
    }
    fn get_type_comment(&self, type_name: &TypeName) -> Option<String> {
        let comments = self.comment_store.get_type_comment(type_name)?;
        Some(comments.iter().fold(String::new(), |acc, cur| {
            format!("{}{}{}\n", acc, self.comment_prefix, cur)
        }))
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
        let attributes = self
            .attribute_store
            .get_property_attribute(type_name, property_key)?;
        Some(
            attributes
                .iter()
                .fold(String::new(), |acc, cur| format!("{}{}\n", acc, cur)),
        )
    }
    fn get_type_attribute(&self, type_name: &TypeName) -> Option<String> {
        let attributes = self.attribute_store.get_type_attribute(type_name)?;
        Some(
            attributes
                .iter()
                .fold(String::new(), |acc, cur| format!("{}{}\n", acc, cur)),
        )
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
        ) -> Option<&'static str> {
            Some("public")
        }
        fn get_type_visibility(&self, _type_name: &TypeName) -> Option<&'static str> {
            Some("public")
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
        ) -> Option<&'static str> {
            None
        }
        fn get_type_visibility(&self, _type_name: &TypeName) -> Option<&'static str> {
            None
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
