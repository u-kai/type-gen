use super::{
    filed_attr::FiledAttribute, filed_comment::FiledComment, filed_visibility::FiledVisibility,
};

pub trait FiledStatement {
    fn create_statement(
        &self,
        comment: impl FiledComment,
        attr: impl FiledAttribute,
        visibility: impl FiledVisibility,
    ) -> String;
}
