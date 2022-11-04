use std::{cell::RefCell, fs::set_permissions, rc::Rc};

use npc::convertor::NamingPrincipalConvertor;

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
    const FILED_DERIMTA: &'static str = ",\n    ";
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
    fn is_option(&self, key: &str) -> bool {
        true
    }
    fn is_pub_struct(&self) -> bool {
        self.pub_structs.contains(&self.struct_name)
    }
    fn is_pub_filed(&self, key: &String) -> bool {
        self.pub_fileds.contains(key)
    }
    fn spawn_child(&self, child_name: impl Into<String>) -> RustStructConvertor {
        Self {
            struct_name: child_name.into(),
            reserveds: self.reserveds.clone(),
            derives: self.derives.clone(),
            pub_structs: self.pub_structs.clone(),
            pub_fileds: self.pub_fileds.clone(),
        }
    }
    fn ket_to_child_name(&self, key: &str) -> String {
        let child_name = NamingPrincipalConvertor::new(key).to_pascal();
        format!("{}{}", self.struct_name, child_name)
    }
    fn pub_struct_line(&self) -> String {
        format!("pub {}", self.struct_line())
    }
    fn struct_line(&self) -> String {
        format!("struct {} {{\n    ", self.struct_name)
    }
    fn derive_statement(&self) -> String {
        let derives = self.derives.join(",");
        format!("#[derive({})]", derives)
    }
}

impl JsonTypeConvertor for RustStructConvertor {
    fn case_object_with_key(
        &self,
        key: &str,
        obj: &std::collections::BTreeMap<String, Json>,
        stack: &mut Vec<String>,
    ) -> String {
        String::new()
    }
    fn case_object(
        &self,
        obj: &std::collections::BTreeMap<String, Json>,
        stack: &mut Vec<String>,
    ) -> String {
        let mut result = if self.is_pub_struct() {
            format!("{}\n{}", self.derive_statement(), self.pub_struct_line())
        } else {
            format!("{}\n{}", self.derive_statement(), self.struct_line())
        };
        for key in obj.keys() {
            let child_object = &obj[key];
            let child_object_value = match child_object {
                Json::String(s) => self.case_string_with_key(key, s),
                Json::Number(num) => self.case_number_with_key(key, num),
                Json::Array(arr) => self.case_array_with_key(key, arr, stack),
                Json::Boolean(bool) => self.case_boolean_with_key(key, *bool),
                Json::Null => self.case_null_with_key(key),
                Json::Object(obj) => {
                    let child_name = self.ket_to_child_name(key);
                    let child_convertor = self.spawn_child(child_name.as_str());
                    let child = child_convertor.case_object(obj, stack);
                    stack.push(child);
                    if self.is_option(key) {
                        format!("Option<{}>", child_name)
                    } else {
                        child_name
                    }
                }
            };
            result = if self.is_pub_filed(&key) {
                format!(
                    "{}pub {}: {}{}",
                    result,
                    self.reserveds.get_or_origin(key),
                    child_object_value,
                    Self::FILED_DERIMTA
                )
            } else {
                format!(
                    "{}{}: {}{}",
                    result,
                    self.reserveds.get_or_origin(key),
                    child_object_value,
                    Self::FILED_DERIMTA
                )
            }
        }
        let result = format!("{}}}", &result[..(result.len() - 4)]);
        result
    }
    fn case_number_with_key(&self, key: &str, num: &serde_json::Number) -> String {
        if self.is_option(key) {
            format!("Option<{}>", self.case_number(num))
        } else {
            self.case_number(num)
        }
    }
    fn case_null_with_key(&self, key: &str) -> String {
        if self.is_option(key) {
            format!("Option<{}>", self.case_null())
        } else {
            self.case_null()
        }
    }
    fn case_array_with_key(&self, key: &str, arr: &Vec<Json>, stack: &mut Vec<String>) -> String {
        if arr.len() == 0 {
            println!(
                "{} can not define. because array is empty ",
                self.struct_name
            );
            return String::new();
        }
        if key == "" {
            todo!("not impl yet")
        }
        let represent = &arr[0];
        match represent {
            Json::Object(obj) => {
                let child_name = self.ket_to_child_name(key);
                let child = self.spawn_child(&child_name);
                let child_value = child.case_object(obj, stack);
                stack.push(child_value);
                if self.is_option(key) {
                    format!("Option<Vec<{}>>", child_name)
                } else {
                    format!("Vec<{}>", child_name)
                }
            }
            Json::Array(arr) => self.case_array_with_key(&format!("Vec<{}>", key), arr, stack),
            Json::Null => self.case_null(),
            Json::Boolean(_) => {
                if self.is_option(key) {
                    String::from("Option<Vec<bool>>")
                } else {
                    String::from("Vec<bool>")
                }
            }
            Json::String(_) => {
                if self.is_option(key) {
                    String::from("Option<Vec<String>>")
                } else {
                    String::from("Vec<String>")
                }
            }
            Json::Number(num) => {
                if self.is_option(key) {
                    if num.is_f64() {
                        return String::from("Option<Vec<f64>>");
                    }
                    if num.is_i64() {
                        return String::from("Option<Vec<isize>>");
                    }
                    if num.is_u64() {
                        return String::from("Option<Vec<usize>>");
                    }
                }
                if num.is_f64() {
                    return String::from("Vec<f64>");
                }
                if num.is_i64() {
                    return String::from("Vec<isize>");
                }
                if num.is_u64() {
                    return String::from("Vec<usize>");
                }
                panic!("{} is not impl", num)
            }
        }
    }
    fn case_boolean_with_key(&self, key: &str, bool: bool) -> String {
        if self.is_option(key) {
            format!("Option<{}>", self.case_boolean(bool))
        } else {
            self.case_boolean(bool)
        }
    }
    fn case_array(&self, arr: &Vec<Json>, stack: &mut Vec<String>) -> String {
        String::from("todo")
    }
    fn case_string_with_key(&self, key: &str, str: &str) -> String {
        if self.is_option(key) {
            format!("Option<{}>", self.case_string(str))
        } else {
            self.case_string(str)
        }
    }
    fn case_number(&self, num: &serde_json::Number) -> String {
        if num.is_f64() {
            return String::from("f64");
        }
        if num.is_i64() {
            return String::from("isize");
        }
        if num.is_u64() {
            return String::from("usize");
        }
        panic!("not considering to {}", num);
    }
    fn case_string(&self, _: &str) -> String {
        String::from("String")
    }
    fn case_null(&self) -> String {
        String::from("String")
    }
    fn case_boolean(&self, _: bool) -> String {
        String::from("bool")
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
    id: Option<isize>,
    test: Option<String>,
}
#[derive(Serialize,Desrialize)]
pub struct TestJsonDataEntities {
    id: Option<isize>,
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
