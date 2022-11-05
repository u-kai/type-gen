use super::{
    filed_attr::FiledAttribute, filed_comment::FiledComment, filed_visibility::FiledVisibility,
    reserved_words::ReservedWords,
};

pub trait FiledStatement<C, A, V, R>
where
    C: FiledComment,
    A: FiledAttribute,
    V: FiledVisibility,
    R: ReservedWords,
{
    const HEAD_SPACE: &'static str = "    ";
    const FILED_DERIMITA: &'static str = ",";
    fn create_statement(
        &self,
        filed_key: &str,
        filed_type: &str,
        comment: &C,
        attr: &A,
        visibility: &V,
        reserved: &R,
    ) -> String;
    fn add_head_space(&self, statement: String) -> String {
        format!("{}{}", Self::HEAD_SPACE, statement)
    }
}
