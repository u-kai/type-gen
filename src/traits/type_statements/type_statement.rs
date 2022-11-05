use crate::traits::off_side_rule::OffSideRule;

use super::{
    type_attr::TypeAttribution, type_comment::TypeComment, type_visibility::TypeVisibility,
};

pub trait TypeStatement<C, A, V, O>
where
    C: TypeComment,
    A: TypeAttribution,
    V: TypeVisibility,
    O: OffSideRule,
{
    const TYPE_STATEMENT: &'static str;
    fn create_statement(
        &self,
        type_key: &str,
        comment: &C,
        attr: &A,
        visi: &V,
        off_side_rule: &O,
    ) -> String {
        let visi = visi.get_visibility_str(type_key);
        let rule = off_side_rule.start();
        let mut result = format!("{}{} {} {}", visi, Self::TYPE_STATEMENT, type_key, rule);

        if let Some(attr) = attr.get_attr(type_key) {
            result = format!("{}{}", attr, result);
        };
        if let Some(comment) = comment.get_comment(type_key) {
            result = format!("{}{}", comment, result);
        };
        result
    }
}
