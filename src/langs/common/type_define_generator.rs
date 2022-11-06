use std::{cell::RefCell, collections::BTreeMap};

use npc::convertor::NamingPrincipalConvertor;

use crate::{
    json::Json,
    traits::{
        filed_statements::filed_statement::FiledStatement, json_lang_mapper::JsonLangMapper,
        off_side_rule::OffSideRule, optional_checker::OptionalChecker,
        type_statements::type_statement::TypeStatement,
    },
};

use super::optional_checker::BaseOptionalChecker;

pub type TypeDefine = String;
pub struct TypeDefineGenerator<M, T, F, O>
where
    M: JsonLangMapper,
    T: TypeStatement,
    F: FiledStatement,
    O: OffSideRule,
{
    root_type: String,
    type_defines: RefCell<Vec<TypeDefine>>,
    optional_checker: BaseOptionalChecker,
    mapper: M,
    type_statement: T,
    filed_statement: F,
    off_side_rule: O,
}

impl<M, T, F, O> TypeDefineGenerator<M, T, F, O>
where
    M: JsonLangMapper,
    T: TypeStatement,
    F: FiledStatement,
    O: OffSideRule,
{
    pub fn new(
        root_type: impl Into<String>,
        mapper: M,
        type_statement: T,
        filed_statement: F,
        off_side_rule: O,
        optional_checker: BaseOptionalChecker,
    ) -> Self {
        Self {
            root_type: root_type.into(),
            type_defines: RefCell::new(Vec::new()),
            optional_checker,
            mapper,
            type_statement,
            filed_statement,
            off_side_rule,
        }
    }
    pub fn gen_from_json(self, json: &str) -> TypeDefine {
        let json = Json::from(json);
        match json {
            Json::String(_) => self.mapper.case_string().to_string(),
            Json::Null => self.mapper.case_null().to_string(),
            Json::Number(num) => self.mapper.case_num(&num),
            Json::Boolean(_) => self.mapper.case_bool().to_string(),
            Json::Array(arr) => self.case_arr(arr),
            Json::Object(obj) => {
                self.case_obj(&self.root_type, &obj);
                self.type_defines
                    .into_inner()
                    .into_iter()
                    .rev()
                    .reduce(|acc, cur| format!("{}\n\n{}", acc, cur))
                    .unwrap()
            }
        }
    }
    fn case_obj(&self, type_key: &str, obj: &BTreeMap<String, Json>) {
        let mut result = format!(
            "{} {}",
            self.type_statement.create_statement(type_key),
            self.off_side_rule.start()
        );
        let keys = obj.keys();
        for key in keys {
            let filed_type_str = match &obj[key] {
                Json::String(_) => {
                    if self.optional_checker.is_optional(type_key, key.as_str()) {
                        self.mapper.make_optional_type(self.mapper.case_string())
                    } else {
                        self.mapper.case_string().to_string()
                    }
                }
                Json::Null => {
                    if self.optional_checker.is_optional(type_key, key.as_str()) {
                        self.mapper.make_optional_type(self.mapper.case_null())
                    } else {
                        self.mapper.case_null().to_string()
                    }
                }
                Json::Number(num) => {
                    if self.optional_checker.is_optional(type_key, key.as_str()) {
                        self.mapper.make_optional_type(&self.mapper.case_num(num))
                    } else {
                        self.mapper.case_num(num)
                    }
                }
                Json::Boolean(_) => {
                    if self.optional_checker.is_optional(type_key, key.as_str()) {
                        self.mapper.make_optional_type(self.mapper.case_bool())
                    } else {
                        self.mapper.case_bool().to_string()
                    }
                }
                Json::Object(obj) => {
                    let npc = NamingPrincipalConvertor::new(key);
                    let child_type_key = format!("{}{}", type_key, npc.to_pascal());
                    self.case_obj(&child_type_key, obj);
                    if self.optional_checker.is_optional(type_key, key.as_str()) {
                        self.mapper.make_optional_type(&child_type_key)
                    } else {
                        child_type_key
                    }
                }
                Json::Array(arr) => self.case_arr_with_key(type_key, key, arr),
            };
            result = format!(
                "{}{}\n",
                result,
                self.filed_statement.create_statement(&key, &filed_type_str)
            )
        }
        result.push_str(self.off_side_rule.end());
        self.type_defines.borrow_mut().push(result);
    }
    fn case_arr(&self, _: Vec<Json>) -> String {
        //self.mapper.make_array_type(type_str)
        todo!("case arr")
    }
    fn case_arr_with_key(&self, type_key: &str, key: &str, arr: &Vec<Json>) -> String {
        if arr.len() == 0 {
            println!("{} can not define. because array is empty ", self.root_type);
            return String::new();
        }
        let represent = &arr[0];
        match represent {
            Json::String(_) => {
                let array_type = self.mapper.make_array_type(self.mapper.case_string());
                if self.optional_checker.is_optional(type_key, key) {
                    self.mapper.make_optional_type(&array_type)
                } else {
                    array_type
                }
            }
            Json::Null => {
                let array_type = self.mapper.make_array_type(self.mapper.case_null());
                if self.optional_checker.is_optional(type_key, key) {
                    self.mapper.make_optional_type(&array_type)
                } else {
                    array_type
                }
            }
            Json::Number(num) => {
                let array_type = self.mapper.make_array_type(&self.mapper.case_num(num));
                if self.optional_checker.is_optional(type_key, key) {
                    self.mapper.make_optional_type(&array_type)
                } else {
                    array_type
                }
            }
            Json::Boolean(_) => {
                let array_type = self.mapper.make_array_type(self.mapper.case_bool());
                if self.optional_checker.is_optional(type_key, key) {
                    self.mapper.make_optional_type(&array_type)
                } else {
                    array_type
                }
            }
            Json::Array(arr) => self.case_arr_with_key(type_key, key, arr),
            Json::Object(obj) => {
                let npc = NamingPrincipalConvertor::new(key);
                let child_type_key = format!("{}{}", type_key, npc.to_pascal());
                self.case_obj(&child_type_key, obj);
                let array_type = self.mapper.make_array_type(&child_type_key);
                if self.optional_checker.is_optional(type_key, key) {
                    self.mapper.make_optional_type(&array_type)
                } else {
                    array_type
                }
            }
        }
    }
}

#[cfg(test)]
mod test_type_define_gen {

    use crate::langs::{
        common::{filed_comment::BaseFiledComment, type_comment::BaseTypeComment},
        rust::{
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
        },
    };

    use super::*;
    #[test]
    fn test_case_rust() {
        let json = r#"
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
        let mut optional_checker = BaseOptionalChecker::default();
        optional_checker.add_require("TestJson", "data");
        let t_comment = BaseTypeComment::new("//");
        let mut t_attr = RustTypeAttributeStore::new();
        t_attr.add_attr(
            "TestJson",
            RustTypeAttribute::Derive(vec!["Serialize".to_string(), "Desrialize".to_string()]),
        );
        t_attr.add_attr(
            "TestJsonData",
            RustTypeAttribute::Derive(vec!["Serialize".to_string(), "Desrialize".to_string()]),
        );
        t_attr.add_attr(
            "TestJsonDataEntities",
            RustTypeAttribute::Derive(vec!["Serialize".to_string(), "Desrialize".to_string()]),
        );
        let mut t_visi = RustTypeVisibilityProvider::new();
        t_visi.add_visibility("TestJson", RustVisibility::Public);
        t_visi.add_visibility("TestJsonDataEntities", RustVisibility::Public);
        let rw = RustReservedWords::new();
        let f_comment = BaseFiledComment::new("//");
        let f_attr = RustFiledAttributeStore::new();
        let f_visi = RustFiledVisibilityProvider::new();
        let osr = RustOffSideRule::new();
        let mapper = JsonRustMapper::new();
        let t_statement = RustTypeStatement::new(t_comment, t_visi, t_attr);
        let f_statement = RustFiledStatement::new(f_comment, RefCell::new(f_attr), f_visi, rw);
        let rust = TypeDefineGenerator::new(
            "TestJson",
            mapper,
            t_statement,
            f_statement,
            osr,
            optional_checker,
        );
        assert_eq!(rust.gen_from_json(json), tobe);
    }
}
