use crate::{convertor::JsonTypeConvertor, json::Json};

use super::reserved_words::ReservedWords;

pub struct RustStructConvertor {
    struct_name: String,
    reserveds: ReservedWords,
    derives: Vec<String>,
    pub_structs: Vec<String>,
    pub_fileds: Vec<String>,
}

impl RustStructConvertor {
    pub fn new(
        struct_name: String,
        derives: Vec<String>,
        pub_structs: Vec<String>,
        pub_fileds: Vec<String>,
    ) -> Self {
        Self {
            struct_name,
            reserveds: ReservedWords::new(),
            derives,
            pub_structs,
            pub_fileds,
        }
    }
}

impl JsonTypeConvertor for RustStructConvertor {
    fn case_string_with_key(&self, key: &str, str: &str) -> String {
        String::new()
    }
    fn case_string(&self, str: &str) -> String {
        String::new()
    }
    fn case_object_with_key(
        &self,
        key: &str,
        obj: &std::collections::BTreeMap<String, Json>,
    ) -> String {
        String::new()
    }
    fn case_object(&self, obj: &std::collections::BTreeMap<String, Json>) -> String {
        String::new()
    }
    fn case_number_with_key(&self, key: &str, num: &serde_json::Number) -> String {
        String::new()
    }
    fn case_number(&self, num: &serde_json::Number) -> String {
        String::new()
    }
    fn case_null_with_key(&self, key: &str) -> String {
        String::new()
    }
    fn case_null(&self) -> String {
        String::new()
    }
    fn case_array(&self, arr: &Vec<Json>) -> String {
        String::new()
    }
    fn case_array_with_key(&self, key: &str, arr: &Vec<Json>) -> String {
        String::new()
    }
    fn case_boolean(&self, bool: bool) -> String {
        String::new()
    }
    fn case_boolean_with_key(&self, key: &str, bool: bool) -> String {
        String::new()
    }
}
#[cfg(test)]
mod rust_type_convertor {
    use crate::rust::builder::RustStructConvertorBuilder;

    use super::*;
    const FIELD_SPACE: &str = "\n    ";
    #[test]
    fn test_set_pub_struct() {
        let complicated_json = r#"
{
    "data":[
        {
            "id":12345,
            "test":"test-string",
            "entities":{
                "id":0
            }
        }
    ]
}
"#;
        let struct_name = "TestJson";
        let tobe = r#"#[derive(Serialize,Desrialize)]
pub struct TestJson {
    data: Option<Vec<TestJsonData>>,
}
#[derive(Serialize,Desrialize)]
struct TestJsonData {
    entities: Option<TestJsonDataEntities>,
    id: Option<f64>,
    test: Option<String>,
}
#[derive(Serialize,Desrialize)]
pub struct TestJsonDataEntities {
    id: Option<f64>,
}"#
        .to_string();
        let convertor = RustStructConvertorBuilder::new_with_derives(
            struct_name,
            vec!["Serialize", "Desrialize"],
        )
        .set_pub_struct("TestJson")
        .set_pub_struct("TestJsonDataEntities")
        .build();
        assert_eq!(convertor.gen_from_json_example(complicated_json), tobe);
    }
}
