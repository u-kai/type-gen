use lang_common::{
    type_defines::additional_defines::{
        additional_statement::AdditionalStatementProvider, attribute_store::Attribute,
        comment_store::Comment, visibility_store::Visibility,
    },
    types::type_name::TypeName,
};

pub(crate) struct TypeDefineConfiguration<V, C, A>
where
    V: Visibility,
    C: Comment,
    A: Attribute,
{
    root_name: TypeName,
    all_type_visibility: Option<V>,
    all_property_visibility: Option<V>,
    all_type_comment: Option<C>,
    all_property_comment: Option<C>,
    all_type_attribute: Option<A>,
    all_property_attribute: Option<A>,
    all_property_is_optional: bool,
}

impl<V, C, A> TypeDefineConfiguration<V, C, A>
where
    V: Visibility,
    C: Comment,
    A: Attribute,
{
}
