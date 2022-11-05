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
    fn create_statement(&self, struct_name: &str) -> String;
}
