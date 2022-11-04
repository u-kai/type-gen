use crate::traits::{
    filed_statements::{
        filed_attr::FiledAttribute, filed_comment::FiledComment, filed_statement::FiledStatement,
        filed_visibility::FiledVisibility, naming_principal_changer::NamingPrincipalChanger,
        optional_checker::OptionalChecker, reserved_words::ReservedWords,
    },
    json_lang_mapper::{
        array::PrimitiveArray, optional::OptionalPrimitive, optional_array::OptionalPrimitiveArray,
        primitive::Primitive,
    },
};

//pub trait LangConfig {
//fn filed_attr(&self) -> &impl FiledAttribute;
//fn filed_comment(&self) -> &impl FiledComment;
//fn filed_statement(&self) -> &impl FiledStatement;
//fn filed_visibility(&self) -> &impl FiledVisibility;
//fn naming_principal_changer(&self) -> &impl NamingPrincipalChanger;
//fn optional_checker(&self) -> &impl OptionalChecker;
//fn reserved_words(&self) -> &impl ReservedWords;
//fn mapper(&self) -> &impl Primitive;
//fn array_mapper(&self) -> &impl PrimitiveArray;
//fn optional_array_mapper(&self) -> &impl OptionalPrimitiveArray;
//fn optional_mapper(&self) -> &impl OptionalPrimitive;
//}
