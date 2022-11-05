use super::{
    type_attr::TypeAttribution, type_comment::TypeComment, type_visibility::TypeVisibility,
};

pub trait TypeStatement<'a> {
    fn create_statement(
        &self,
        comment: impl TypeComment,
        attr: impl TypeAttribution,
        visibility: impl TypeVisibility,
    ) -> String;
}
