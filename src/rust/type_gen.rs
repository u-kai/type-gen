use std::{cell::RefCell, collections::BTreeMap};

use npc::convertor::NamingPrincipalConvertor;

use crate::{
    json::Json,
    lang_common::{filed_comment::BaseFiledComment, type_comment::BaseTypeComment},
    traits::json_lang_mapper::primitive::Primitive,
};

use super::{
    filed_statements::{
        filed_attr::RustFiledAttributeStore, filed_visibilty::RustFiledVisibilityProvider,
    },
    json_lang_mapper::primitive::RustJsonPrimitiveMapper,
    reserved_words::RustReservedWords,
    rust_visibility::RustVisibility,
    type_statements::{
        type_attr::{RustTypeAttribute, RustTypeAttributeStore},
        type_visiblity::RustTypeVisibilityProvider,
    },
};
struct RustFiledStatements {
    reserved_words: RustReservedWords,
    visi: RustFiledVisibilityProvider,
    attr: RustFiledAttributeStore,
    comment: BaseFiledComment,
}
struct RustTypeStatements {
    visi: RustTypeVisibilityProvider,
    attr: RustTypeAttributeStore,
    comment: BaseTypeComment,
}

pub struct RustTypeGenerator {
    struct_name: String,
    obj_str_stack: RefCell<Vec<String>>,
    type_statements: RustTypeStatements,
    filed_statements: RustFiledStatements,
}

impl RustTypeGenerator {
    pub fn new(struct_name: &str) -> Self {
        let type_s = RustTypeStatements {
            visi: RustTypeVisibilityProvider::new(),
            attr: RustTypeAttributeStore::new(),
            comment: BaseTypeComment::new("//"),
        };
        let filed_s = RustFiledStatements {
            visi: RustFiledVisibilityProvider::new(),
            attr: RustFiledAttributeStore::new(),
            comment: BaseFiledComment::new("//"),
            reserved_words: RustReservedWords::new(),
        };
        Self {
            struct_name: struct_name.to_string(),
            obj_str_stack: RefCell::new(Vec::new()),
            type_statements: type_s,
            filed_statements: filed_s,
        }
    }
    pub fn from_json_example(&self, json: &str) -> String {
        let json = Json::from(json);
        match json {
            Json::String(_) => RustJsonPrimitiveMapper::new().case_string().to_string(),
            Json::Null => RustJsonPrimitiveMapper::new().case_null().to_string(),
            Json::Number(num) => {
                if num.is_f64() {
                    return RustJsonPrimitiveMapper::new().case_f64().to_string();
                }
                if num.is_i64() {
                    return RustJsonPrimitiveMapper::new().case_i64().to_string();
                }
                RustJsonPrimitiveMapper::new().case_u64().to_string()
            }
            Json::Boolean(_) => RustJsonPrimitiveMapper::new().case_bool().to_string(),
            Json::Array(arr) => self.case_arr(arr),
            Json::Object(obj) => self.case_obj(obj),
        }
    }
    pub fn case_obj(&self, obj: BTreeMap<String, Json>) -> String {
        let keys = obj.keys();
        for key in keys {
            let value = &obj[key];
            let child_struct_name = format!(
                "{}{}",
                self.struct_name,
                NamingPrincipalConvertor::new(key).to_pascal()
            );
        }
        String::new()
    }
    pub fn case_arr(&self, arr: Vec<Json>) -> String {
        String::new()
    }
    pub fn case_arr_with_key(&self, key: &str, arr: Vec<Json>) -> String {
        String::new()
    }
    pub fn set_pub_struct(&mut self, struct_name: &str) {
        self.type_statements
            .visi
            .add_visibility(struct_name, RustVisibility::Public);
    }
    pub fn add_derives(&mut self, struct_name: &str, derives: Vec<&str>) {
        self.type_statements.attr.set_attr(
            struct_name,
            RustTypeAttribute::Derive(derives.iter().map(|s| s.to_string()).collect::<Vec<_>>()),
        )
    }
}

#[cfg(test)]
mod test_rust_type_gen {
    use super::*;
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
        let mut rust = RustTypeGenerator::new("TestJson");
        rust.add_derives("TestJson", vec!["Serialize", "Desrialize"]);
        rust.add_derives("TestJsonData", vec!["Serialize", "Desrialize"]);
        rust.add_derives("TestJsonDataEntities", vec!["Serialize", "Desrialize"]);
        rust.set_pub_struct("TestJson");
        rust.set_pub_struct("TestJsonDataEntities");
        //assert_eq!(rust.from_json_example(complicated_json), tobe);
    }
}
