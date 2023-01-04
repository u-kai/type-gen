use std::{collections::BTreeMap, fs::read_to_string, path::Path};

use lang_common::type_defines::{
    additional_defines::{
        additional_statement::AdditionalStatement, attribute_store::Attribute,
        comment_store::Comment, visibility_store::Visibility,
    },
    builder::TypeDefineBuilder,
    generators::{
        generator::{PropertyStatementGenerator, TypeDefineGenerator, TypeStatementGenerator},
        mapper::LangTypeMapper,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigJson {
    src: IOConfig,
    dist: IOConfig,
    comment: Option<CommentConfig>,
    attribute: Option<AttributeConfig>,
    visibility: Option<VisibilityConfig>,
    optional: Option<OptionalConfig>,
}
impl ConfigJson {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let json = read_to_string(path).unwrap();
        let json: ConfigJson = serde_json::from_str(&json).unwrap();
        json
    }
    pub fn dist_extension(&self) -> &str {
        &self.dist.extension
    }
    pub fn src(&self) -> &str {
        &self.src.root
    }
    pub fn dist(&self) -> &str {
        &self.dist.root
    }
    pub fn to_definer<T, P, M, A, V, C, At>(
        &self,
        mut builder: impl TypeDefineBuilder<T, P, M, A, V, C, At>,
    ) -> TypeDefineGenerator<T, P, M, A>
    where
        T: TypeStatementGenerator<M>,     // A>,
        P: PropertyStatementGenerator<M>, //, A>,
        M: LangTypeMapper,
        A: AdditionalStatement,
        V: Visibility,
        C: Comment,
        At: Attribute,
    {
        if self.all_type_attribute().is_some() {
            builder = builder
                .set_all_type_attribute(self.all_type_attribute().unwrap().to_string().into());
        };
        if self.all_property_attribute().is_some() {
            builder = builder.set_all_property_attribute(
                self.all_property_attribute().unwrap().to_string().into(),
            );
        };
        if self.all_type_visibility().is_some() {
            builder = builder
                .set_all_type_visibility(self.all_type_visibility().unwrap().to_string().into());
        };
        if self.all_property_visibility().is_some() {
            builder = builder.set_all_property_visibility(
                self.all_property_visibility().unwrap().to_string().into(),
            );
        };
        if self.all_type_comment().is_some() {
            builder =
                builder.set_all_type_comment(self.all_type_comment().unwrap().to_string().into());
        };
        if self.all_property_comment().is_some() {
            builder = builder
                .set_all_property_comment(self.all_property_comment().unwrap().to_string().into());
        };
        if self.all_optional().is_some() {
            builder = builder.set_all_type_optional(self.all_optional().unwrap());
        }
        builder.build()
    }
    fn all_optional(&self) -> Option<bool> {
        self.optional.as_ref()?.all
    }
    fn all_type_attribute(&self) -> Option<&str> {
        self.attribute
            .as_ref()?
            .alltype
            .as_ref()
            .map(|s| s.as_str())
    }
    fn all_property_attribute(&self) -> Option<&str> {
        self.attribute
            .as_ref()?
            .allproperty
            .as_ref()
            .map(|s| s.as_str())
    }
    fn all_type_comment(&self) -> Option<&str> {
        self.comment.as_ref()?.alltype.as_ref().map(|s| s.as_str())
    }
    fn all_property_comment(&self) -> Option<&str> {
        self.comment
            .as_ref()?
            .allproperty
            .as_ref()
            .map(|s| s.as_str())
    }
    fn all_type_visibility(&self) -> Option<&str> {
        self.visibility
            .as_ref()?
            .alltype
            .as_ref()
            .map(|s| s.as_str())
    }
    fn all_property_visibility(&self) -> Option<&str> {
        self.visibility
            .as_ref()?
            .allproperty
            .as_ref()
            .map(|s| s.as_str())
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct IOConfig {
    root: String,
    extension: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct OptionalConfig {
    all: Option<bool>,
    default: Option<bool>,
    property: Option<BTreeMap<String, BTreeMap<String, bool>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AttributeConfig {
    r#type: Option<TypeConfig>,
    property: Option<PropertyConfig>,
    alltype: Option<String>,
    allproperty: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct VisibilityConfig {
    r#type: Option<TypeConfig>,
    property: Option<PropertyConfig>,
    alltype: Option<String>,
    allproperty: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CommentConfig {
    r#type: Option<TypeConfig>,
    property: Option<PropertyConfig>,
    alltype: Option<String>,
    allproperty: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TypeConfig(BTreeMap<String, String>);

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PropertyConfig(BTreeMap<String, BTreeMap<String, String>>);
