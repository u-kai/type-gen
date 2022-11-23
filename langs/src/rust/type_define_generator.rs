use lang_common::type_defines::generators::generator::TypeStatementGenerator;

pub struct RustTypeStatementGenerator {}
//impl TypeStatementGenerator for RustTypeStatementGenerator {
//fn generate_case_composite<A: lang_common::type_defines::additional_defines::additional_statement::AdditionalStatement>(
//&self,
//type_name: &lang_common::types::type_name::TypeName,
//properties_statement: String,
//additional_statement: &A,
//) -> String {
//let mut result = String::new();
//if let Some(comment) = additional_statement.get_type_comment(type_name) {
//result += &comment;
//};
//if let Some(attribute) = additional_statement.get_type_attribute(type_name) {
//result += &attribute;
//};
//format!(
//"{}{} {} {{{}}}",
//result,
//Self::TYPE_PREFIX,
//type_name.as_str(),
//properties_statement
//)

//}
//}
