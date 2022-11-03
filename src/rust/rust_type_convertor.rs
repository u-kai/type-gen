//pub struct RustTypeConvertor {

//}

//#[cfg(test)]
//mod rust_type_convertor {
//use super::*;
//const FIELD_SPACE: &str = "\n    ";
//#[test]
//fn test_set_pub_struct() {
//let complicated_json = r#"
//{
//"data":[
//{
//"id":12345,
//"test":"test-string",
//"entities":{
//"id":0
//}
//}
//]
//}
//"#;
//let struct_name = "TestJson";
//let tobe = r#"#[derive(Serialize,Desrialize)]
//pub struct TestJson {
//data: Option<Vec<TestJsonData>>,
//}
//#[derive(Serialize,Desrialize)]
//struct TestJsonData {
//entities: Option<TestJsonDataEntities>,
//id: Option<f64>,
//test: Option<String>,
//}
//#[derive(Serialize,Desrialize)]
//pub struct TestJsonDataEntities {
//id: Option<f64>,
//}"#
//.to_string();
//let mut convertor =
//RustTypeConvertor::new_with_drives(vec!["Serialize", "Desrialize"], struct_name);
//convertor
//.set_pub_struct("TestJson")
//.set_pub_struct("TestJsonDataEntities");
//assert_eq!(convertor.from_json_example(complicated_json).unwrap(), tobe);
//}
//}
