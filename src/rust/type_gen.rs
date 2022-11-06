use std::{cell::RefCell, collections::BTreeMap};

use npc::{convertor::NamingPrincipalConvertor, naming_principal::NamingPrincipal};

use crate::{
    json::Json,
    lang_common::{
        filed_comment::BaseFiledComment, optional_checker::BaseOptionalChecker,
        type_comment::BaseTypeComment,
    },
    traits::{
        filed_statements::filed_statement::FiledStatement, json_lang_mapper::JsonLangMapper,
        off_side_rule::OffSideRule, optional_checker::OptionalChecker,
        type_statements::type_statement::TypeStatement,
    },
};

use super::{
    filed_statements::{
        filed_attr::RustFiledAttributeStore, filed_statement::RustFiledStatement,
        filed_visibilty::RustFiledVisibilityProvider, reserved_words::RustReservedWords,
    },
    json_lang_mapper::JsonRustMapper,
    off_side_rule::RustOffSideRule,
    rust_visibility::RustVisibility,
    type_statements::{
        type_attr::{RustTypeAttribute, RustTypeAttributeStore},
        type_statement::RustTypeStatement,
        type_visiblity::RustTypeVisibilityProvider,
    },
};
struct RustFiledStatements {
    reserved_words: RustReservedWords,
    visi: RustFiledVisibilityProvider,
    attr: RefCell<RustFiledAttributeStore>,
    comment: BaseFiledComment,
}
struct RustTypeStatements {
    visi: RustTypeVisibilityProvider,
    attr: RustTypeAttributeStore,
    comment: BaseTypeComment,
    offside: RustOffSideRule,
}
pub struct RustTypeGenerator {
    struct_name: String,
    optional_checker: BaseOptionalChecker,
    obj_str_stack: RefCell<Vec<String>>,
    type_statements: RustTypeStatements,
    filed_statements: RustFiledStatements,
    mapper: JsonRustMapper,
}

impl RustTypeGenerator {
    pub fn new(struct_name: &str) -> Self {
        let type_s = RustTypeStatements {
            visi: RustTypeVisibilityProvider::new(),
            attr: RustTypeAttributeStore::new(),
            comment: BaseTypeComment::new("//"),
            offside: RustOffSideRule::new(),
        };
        let filed_s = RustFiledStatements {
            visi: RustFiledVisibilityProvider::new(),
            attr: RefCell::new(RustFiledAttributeStore::new()),
            comment: BaseFiledComment::new("//"),
            reserved_words: RustReservedWords::new(),
        };

        Self {
            struct_name: struct_name.to_string(),
            optional_checker: BaseOptionalChecker::default(),
            obj_str_stack: RefCell::new(Vec::new()),
            type_statements: type_s,
            filed_statements: filed_s,
            mapper: JsonRustMapper::new(),
        }
    }
    pub fn from_json_example(self, json: &str) -> String {
        let json = Json::from(json);
        match json {
            Json::String(_) => self.mapper.case_string().to_string(),
            Json::Null => self.mapper.case_null().to_string(),
            Json::Number(num) => self.mapper.case_num(&num),
            Json::Boolean(_) => self.mapper.case_bool().to_string(),
            Json::Array(arr) => self.case_arr(arr),
            Json::Object(obj) => {
                self.case_obj(&self.struct_name, &obj);
                self.obj_str_stack
                    .into_inner()
                    .into_iter()
                    .rev()
                    .reduce(|acc, cur| format!("{}\n\n{}", acc, cur))
                    .unwrap()
            }
        }
    }
    fn case_obj(&self, struct_name: &str, obj: &BTreeMap<String, Json>) {
        let mut result = RustTypeStatement::new().create_statement(
            struct_name,
            &self.type_statements.comment,
            &self.type_statements.attr,
            &self.type_statements.visi,
            &self.type_statements.offside,
        );
        let keys = obj.keys();
        for key in keys {
            let filed_type_str = match &obj[key] {
                Json::String(_) => {
                    if self.optional_checker.is_optional(struct_name, key.as_str()) {
                        self.mapper.make_optional_type(self.mapper.case_string())
                    } else {
                        self.mapper.case_string().to_string()
                    }
                }
                Json::Null => {
                    if self.optional_checker.is_optional(struct_name, key.as_str()) {
                        self.mapper.make_optional_type(self.mapper.case_null())
                    } else {
                        self.mapper.case_null().to_string()
                    }
                }
                Json::Number(num) => {
                    if self.optional_checker.is_optional(struct_name, key.as_str()) {
                        self.mapper.make_optional_type(&self.mapper.case_num(num))
                    } else {
                        self.mapper.case_num(num)
                    }
                }
                Json::Boolean(_) => {
                    if self.optional_checker.is_optional(struct_name, key.as_str()) {
                        self.mapper.make_optional_type(self.mapper.case_bool())
                    } else {
                        self.mapper.case_bool().to_string()
                    }
                }
                Json::Object(obj) => {
                    let npc = NamingPrincipalConvertor::new(key);
                    let child_struct_name = format!("{}{}", struct_name, npc.to_pascal());
                    self.case_obj(&child_struct_name, obj);
                    if self.optional_checker.is_optional(struct_name, key.as_str()) {
                        self.mapper.make_optional_type(&child_struct_name)
                    } else {
                        child_struct_name
                    }
                }
                Json::Array(arr) => self.case_arr_with_key(struct_name, key, arr),
            };
            result = format!(
                "{}{}\n",
                result,
                RustFiledStatement::new().create_statement(
                    key.as_str(),
                    filed_type_str.as_str(),
                    &self.filed_statements.comment,
                    &mut self.filed_statements.attr.borrow_mut(),
                    &self.filed_statements.visi,
                    &self.filed_statements.reserved_words
                )
            )
        }
        result.push_str(self.type_statements.offside.end());
        self.obj_str_stack.borrow_mut().push(result);
    }
    fn case_arr(&self, arr: Vec<Json>) -> String {
        String::new()
    }
    fn case_arr_with_key(&self, struct_name: &str, key: &str, arr: &Vec<Json>) -> String {
        println!("{}", struct_name);
        if arr.len() == 0 {
            println!(
                "{} can not define. because array is empty ",
                self.struct_name
            );
            return String::new();
        }
        let represent = &arr[0];
        match represent {
            Json::String(_) => {
                let array_type = self.mapper.make_array_type(self.mapper.case_string());
                if self.optional_checker.is_optional(struct_name, key) {
                    self.mapper.make_optional_type(&array_type)
                } else {
                    array_type
                }
            }
            Json::Null => {
                let array_type = self.mapper.make_array_type(self.mapper.case_null());
                if self.optional_checker.is_optional(struct_name, key) {
                    self.mapper.make_optional_type(&array_type)
                } else {
                    array_type
                }
            }
            Json::Number(num) => {
                let array_type = self.mapper.make_array_type(&self.mapper.case_num(num));
                if self.optional_checker.is_optional(struct_name, key) {
                    self.mapper.make_optional_type(&array_type)
                } else {
                    array_type
                }
            }
            Json::Boolean(_) => {
                let array_type = self.mapper.make_array_type(self.mapper.case_bool());
                if self.optional_checker.is_optional(struct_name, key) {
                    self.mapper.make_optional_type(&array_type)
                } else {
                    array_type
                }
            }
            Json::Array(arr) => self.case_arr_with_key(struct_name, key, arr),
            Json::Object(obj) => {
                let npc = NamingPrincipalConvertor::new(key);
                let child_struct_name = format!("{}{}", struct_name, npc.to_pascal());
                self.case_obj(&child_struct_name, obj);
                let array_type = self.mapper.make_array_type(&child_struct_name);
                if self.optional_checker.is_optional(struct_name, key) {
                    self.mapper.make_optional_type(&array_type)
                } else {
                    array_type
                }
            }
        }
    }
    pub fn add_require(&mut self, struct_name: &'static str, filed_key: &'static str) {
        self.optional_checker.add_require(struct_name, filed_key)
    }
    pub fn set_pub_struct(&mut self, struct_name: &str) {
        self.type_statements
            .visi
            .add_visibility(struct_name, RustVisibility::Public);
    }
    pub fn set_pub_filed(&mut self, filed_name: &str) {
        self.filed_statements
            .visi
            .add_visibility(filed_name, RustVisibility::Public);
    }
    pub fn add_derives(&mut self, struct_name: &str, derives: Vec<&str>) {
        self.type_statements.attr.add_attr(
            struct_name,
            RustTypeAttribute::Derive(derives.iter().map(|s| s.to_string()).collect::<Vec<_>>()),
        )
    }
}

#[cfg(test)]
mod test_rust_type_gen {
    use super::*;
    #[test]
    fn test_add_optional() {
        let complicated_json = r#"
            {
                "data":[
                    {
                        "userId":12345,
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
    data: Vec<TestJsonData>,
}

#[derive(Serialize,Desrialize)]
struct TestJsonData {
    entities: Option<TestJsonDataEntities>,
    test: Option<String>,
    #[serde(rename = "userId")]
    user_id: Option<i64>,
}

#[derive(Serialize,Desrialize)]
pub struct TestJsonDataEntities {
    id: Option<i64>,
}"#
        .to_string();
        let mut rust = RustTypeGenerator::new("TestJson");
        rust.add_derives("TestJson", vec!["Serialize", "Desrialize"]);
        rust.add_derives("TestJsonData", vec!["Serialize", "Desrialize"]);
        rust.add_derives("TestJsonDataEntities", vec!["Serialize", "Desrialize"]);
        rust.add_require("TestJson", "data");
        rust.set_pub_struct("TestJson");
        rust.set_pub_struct("TestJsonDataEntities");
        assert_eq!(rust.from_json_example(complicated_json), tobe);
    }
    #[test]
    fn test_rename_serde() {
        let complicated_json = r#"
            {
                "data":[
                    {
                        "userId":12345,
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
    test: Option<String>,
    #[serde(rename = "userId")]
    user_id: Option<i64>,
}

#[derive(Serialize,Desrialize)]
pub struct TestJsonDataEntities {
    id: Option<i64>,
}"#
        .to_string();
        let mut rust = RustTypeGenerator::new("TestJson");
        rust.add_derives("TestJson", vec!["Serialize", "Desrialize"]);
        rust.add_derives("TestJsonData", vec!["Serialize", "Desrialize"]);
        rust.add_derives("TestJsonDataEntities", vec!["Serialize", "Desrialize"]);
        rust.set_pub_struct("TestJson");
        rust.set_pub_struct("TestJsonDataEntities");
        assert_eq!(rust.from_json_example(complicated_json), tobe);
    }
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
    id: Option<i64>,
    test: Option<String>,
}

#[derive(Serialize,Desrialize)]
pub struct TestJsonDataEntities {
    id: Option<i64>,
}"#
        .to_string();
        let mut rust = RustTypeGenerator::new("TestJson");
        rust.add_derives("TestJson", vec!["Serialize", "Desrialize"]);
        rust.add_derives("TestJsonData", vec!["Serialize", "Desrialize"]);
        rust.add_derives("TestJsonDataEntities", vec!["Serialize", "Desrialize"]);
        rust.set_pub_struct("TestJson");
        rust.set_pub_struct("TestJsonDataEntities");
        assert_eq!(rust.from_json_example(complicated_json), tobe);
    }
}
