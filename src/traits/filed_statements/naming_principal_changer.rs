use super::{
    filed_attr::FiledAttribute, filed_comment::FiledComment, filed_statement::FiledStatement,
    filed_visibility::FiledVisibility, reserved_words::ReservedWords,
};

pub trait NamingPrincipalChanger<C, A, V, R>
where
    C: FiledComment,
    A: FiledAttribute,
    V: FiledVisibility,
    R: ReservedWords,
{
    fn change_statement(&self, key: &str, statement: &impl FiledStatement<C, A, V, R>) -> String;
}
