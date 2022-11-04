use super::{
    filed_attr::FiledAttribute, filed_comment::FiledComment, filed_visibility::FiledVisibility,
    optional_checker::OptionalChecker,
};

pub trait FiledStatement {
    fn create_statement(
        &self,
        comment: impl FiledComment,
        attr: impl FiledAttribute,
        visibility: impl FiledVisibility,
        optional: impl OptionalChecker,
    ) -> String;
}
