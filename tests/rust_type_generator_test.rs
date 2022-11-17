//mod test_type_define_gen {

//use std::cell::RefCell;

//use crate::type_defines::generators::from_json::{
//lang_common::{
//field_statements::field_comment::BaseFieldComment,
//optional_checker::BaseOptionalChecker, type_statements::type_comment::BaseTypeComment,
//},
//rust::{
//field_statements::{
//field_attr::RustFieldAttributeStore, field_statement::RustfieldStatement,
//field_visibility::RustFieldVisibilityProvider, reserved_words::RustReservedWords,
//},
//json_lang_mapper::JsonRustMapper,
//off_side_rule::RustOffSideRule,
//rust_visibility::RustVisibility,
//type_statements::{
//type_attr::{RustTypeAttribute, RustTypeAttributeStore},
//type_statement::RustTypeStatement,
//type_visibility::RustTypeVisibilityProvider,
//},
//},
//type_define_generator::TypeDefineGenerator,
//};

//use std::cell::RefCell;

//use type_gen::type_defines::generators::from_json::{
//lang_common::{
//field_statements::field_comment::BaseFieldComment,
//optional_checker::BaseOptionalChecker, type_statements::type_comment::BaseTypeComment,
//},
//rust::{
//field_statements::{
//field_attr::RustFieldAttributeStore, field_visibility::RustFieldVisibilityProvider,
//reserved_words::RustReservedWords,
//},
//json_lang_mapper::JsonRustMapper,
//off_side_rule::RustOffSideRule,
//rust_visibility::RustVisibility,
//type_statements::{
//type_attr::{RustTypeAttribute, RustTypeAttributeStore},
//type_statement::RustTypeStatement,
//type_visibility::RustTypeVisibilityProvider,
//},
//},
//type_define_generator::TypeDefineGenerator,
//};

//#[test]
//fn test_case_rust() {
//let json = r#"
//{
//"data":[
//{
//"userId":12345,
//"test":"test-string",
//"entities":{
//"id":0
//}
//}
//]
//}
//"#;
//let tobe = r#"#[derive(Serialize,Desrialize)]
//pub struct TestJson {
//data: Vec<TestJsonData>,
//}

//#[derive(Serialize,Desrialize)]
//struct TestJsonData {
//entities: Option<TestJsonDataEntities>,
//test: Option<String>,
//#[serde(rename = "userId")]
//user_id: Option<i64>,
//}

//#[derive(Serialize,Desrialize)]
//pub struct TestJsonDataEntities {
//id: Option<i64>,
//}

//"#
//.to_string();
//let mut optional_checker = BaseOptionalChecker::default();
//optional_checker.add_require("TestJson", "data");
//let t_comment = BaseTypeComment::new("//");
//let mut t_attr = RustTypeAttributeStore::new();
//t_attr.add_attr(
//"TestJson",
//RustTypeAttribute::Derive(vec!["Serialize".to_string(), "Desrialize".to_string()]),
//);
//t_attr.add_attr(
//"TestJsonData",
//RustTypeAttribute::Derive(vec!["Serialize".to_string(), "Desrialize".to_string()]),
//);
//t_attr.add_attr(
//"TestJsonDataEntities",
//RustTypeAttribute::Derive(vec!["Serialize".to_string(), "Desrialize".to_string()]),
//);
//let mut t_visi = RustTypeVisibilityProvider::new();
//t_visi.add_visibility("TestJson", RustVisibility::Public);
//t_visi.add_visibility("TestJsonDataEntities", RustVisibility::Public);
//let rw = RustReservedWords::new();
//let f_comment = BaseFieldComment::new("//");
//let f_attr = RustFieldAttributeStore::new();
//let f_visi = RustFieldVisibilityProvider::new();
//let osr = RustOffSideRule::new();
//let mapper = JsonRustMapper::new();
//let t_statement = RustTypeStatement::new(t_comment, t_visi, t_attr);
//let f_statement = RustFieldStatement::new(f_comment, RefCell::new(f_attr), f_visi, rw);
//let rust = TypeDefineGenerator::new(
//"TestJson",
//mapper,
//t_statement,
//f_statement,
//osr,
//optional_checker,
//);
//assert_eq!(rust.gen_from_json(json), tobe);
//}
//}
