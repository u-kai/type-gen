use std::{cell::RefCell, collections::BTreeMap};

use npc::convertor::NamingPrincipalConvertor;

use crate::{
    json::Json,
    traits::{
        filed_statements::filed_statement::FiledStatement, json_lang_mapper::JsonLangMapper,
        off_side_rule::OffSideRule, optional_checker::OptionalChecker,
        type_statements::type_statement::TypeStatement,
    },
    utils::store_fn::{push_to_btree_vec, push_to_kv_vec},
};

use super::optional_checker::BaseOptionalChecker;

pub type TypeDefine = String;

/// TypeDefine is
/// ```
/// struct Test {
///     id:usize,
///     name:String,
/// }
/// ```
/// type_key is struct name -> Test<br>
/// field_key is field name -> id, name...
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
                let root_statement = self.make_child_statement(&self.root_type, obj);
                self.type_defines.borrow_mut().push(root_statement);
                self.type_defines
                    .into_inner()
                    .into_iter()
                    .rev()
                    .reduce(|acc, cur| format!("{}\n\n{}", acc, cur))
                    .unwrap()
            }
        }
    }
    pub fn gen_from_json_(self, json: &str) -> TypeDefine {
        let json = Json::from(json);
        match json {
            Json::String(_) => self.mapper.case_string().to_string(),
            Json::Null => self.mapper.case_null().to_string(),
            Json::Number(num) => self.mapper.case_num(&num),
            Json::Boolean(_) => self.mapper.case_bool().to_string(),
            Json::Array(arr) => self.case_arr(arr),
            Json::Object(obj) => {
                let root_statement = self.make_type_defines_from_obj(&self.root_type, obj);
                self.type_defines.borrow_mut().push(root_statement);
                self.type_defines
                    .into_inner()
                    .into_iter()
                    .rev()
                    .reduce(|acc, cur| format!("{}\n\n{}", acc, cur))
                    .unwrap()
            }
        }
    }
    fn make_type_defines_from_obj(&self, type_key: &str, obj: BTreeMap<String, Json>) -> String {
        let filed_statement = obj
            .into_iter()
            .fold(String::new(), |acc, (filed_key, v)| match v {
                Json::String(_) => {
                    if self
                        .optional_checker
                        .is_optional(type_key, filed_key.as_str())
                    {
                        self.mapper.make_optional_type(self.mapper.case_string())
                    } else {
                        self.mapper.case_string().to_string()
                    }
                }
                Json::Null => {
                    String::new()
                    //if self.optional_checker.is_optional(type_key, filed_key) {
                    //self.mapper.make_optional_type(self.mapper.case_null())
                    //} else {
                    //self.mapper.case_null().to_string()
                    //}
                }
                Json::Number(num) => {
                    String::new()
                    //if self.optional_checker.is_optional(type_key, filed_key) {
                    //self.mapper.make_optional_type(&self.mapper.case_num(&num))
                    //} else {
                    //self.mapper.case_num(&num)
                    //}
                }
                Json::Boolean(_) => {
                    String::new()
                    //if self.optional_checker.is_optional(type_key, filed_key) {
                    //self.mapper.make_optional_type(self.mapper.case_bool())
                    //} else {
                    //self.mapper.case_bool().to_string()
                    //}
                }
                Json::Object(obj) => {
                    let child_type_key = self.child_type_key(type_key, filed_key.as_str());
                    let child_type_statement = self.make_child_statement(&child_type_key, obj);
                    self.type_defines.borrow_mut().push(child_type_statement);
                    if self
                        .optional_checker
                        .is_optional(type_key, filed_key.as_str())
                    {
                        self.mapper.make_optional_type(&child_type_key)
                    } else {
                        child_type_key
                    }
                }
                Json::Array(arr) => self.case_arr_with_key(type_key, filed_key.as_str(), arr),
            });
        String::new()
    }
    fn make_filed_statement_and_staking(
        &self,
        type_key: &str,
        filed_key: &str,
        obj: Json,
    ) -> String {
        let filed_type = match obj {
            Json::String(_) => {
                if self.optional_checker.is_optional(type_key, filed_key) {
                    self.mapper.make_optional_type(self.mapper.case_string())
                } else {
                    self.mapper.case_string().to_string()
                }
            }
            Json::Null => {
                if self.optional_checker.is_optional(type_key, filed_key) {
                    self.mapper.make_optional_type(self.mapper.case_null())
                } else {
                    self.mapper.case_null().to_string()
                }
            }
            Json::Number(num) => {
                if self.optional_checker.is_optional(type_key, filed_key) {
                    self.mapper.make_optional_type(&self.mapper.case_num(&num))
                } else {
                    self.mapper.case_num(&num)
                }
            }
            Json::Boolean(_) => {
                if self.optional_checker.is_optional(type_key, filed_key) {
                    self.mapper.make_optional_type(self.mapper.case_bool())
                } else {
                    self.mapper.case_bool().to_string()
                }
            }
            Json::Object(obj) => {
                let child_type_key = self.child_type_key(type_key, filed_key);
                let child_type_statement = self.make_child_statement(&child_type_key, obj);
                self.type_defines.borrow_mut().push(child_type_statement);
                if self.optional_checker.is_optional(type_key, filed_key) {
                    self.mapper.make_optional_type(&child_type_key)
                } else {
                    child_type_key
                }
            }
            Json::Array(arr) => self.case_arr_with_key(type_key, filed_key, arr),
        };
        self.filed_statement
            .create_statement(filed_key, &filed_type)
    }
    fn make_child_statement(&self, child_type_key: &str, obj: BTreeMap<String, Json>) -> String {
        let child_type_statement = format!(
            "{} {}",
            self.type_statement.create_statement(child_type_key),
            self.off_side_rule.start()
        );
        let mut child_type_statement =
            obj.into_iter()
                .fold(child_type_statement, |acc, (key, value)| {
                    format!(
                        "{}{}\n",
                        acc,
                        self.make_filed_statement_and_staking(child_type_key, key.as_str(), value)
                    )
                });
        child_type_statement.push_str(self.off_side_rule.end());
        child_type_statement
    }

    /// ### array containe some type is not consider example below
    /// \["hello",0,{"name":"kai"}\]<br>
    /// above case is retrun Array(String)<>
    fn case_arr_with_key(&self, type_key: &str, filed_key: &str, arr: Vec<Json>) -> String {
        // case TestObj
        // {test : [{name:kai,age:20},{name:kai},{name:kai,age:20,like:{lang:rust,actor:hamabe}}]};
        // type_key is TestObj
        // filed_key is test
        //
        if arr.len() == 0 {
            println!("{} can not define. because array is empty ", self.root_type);
            return String::new();
        }
        let mut map = BTreeMap::new();
        for obj in arr {
            match obj {
                Json::Object(obj) => {
                    for (k, v) in obj {
                        push_to_btree_vec(&mut map, k, v)
                    }
                }
                Json::String(_) => {
                    let array_type = self.mapper.make_array_type(self.mapper.case_string());
                    return if self.optional_checker.is_optional(type_key, filed_key) {
                        self.mapper.make_optional_type(&array_type)
                    } else {
                        array_type
                    };
                }
                Json::Null => {
                    let array_type = self.mapper.make_array_type(self.mapper.case_null());
                    return if self.optional_checker.is_optional(type_key, filed_key) {
                        self.mapper.make_optional_type(&array_type)
                    } else {
                        array_type
                    };
                }
                Json::Number(num) => {
                    let array_type = self.mapper.make_array_type(&self.mapper.case_num(&num));
                    return if self.optional_checker.is_optional(type_key, filed_key) {
                        self.mapper.make_optional_type(&array_type)
                    } else {
                        array_type
                    };
                }
                Json::Boolean(_) => {
                    let array_type = self.mapper.make_array_type(self.mapper.case_bool());
                    return if self.optional_checker.is_optional(type_key, filed_key) {
                        self.mapper.make_optional_type(&array_type)
                    } else {
                        array_type
                    };
                }
                _ => todo!(),
            }
        }
        // case obj
        let child_type_key = self.child_type_key(type_key, filed_key);
        let child_statement = self.make_child_statement_with_arr(&child_type_key, map);
        self.type_defines.borrow_mut().push(child_statement);
        let array_type = self.mapper.make_array_type(&child_type_key);
        if self.optional_checker.is_optional(type_key, filed_key) {
            self.mapper.make_optional_type(&array_type)
        } else {
            array_type
        }
    }
    fn make_child_statement_with_arr(
        &self,
        child_type_key: &str,
        map: BTreeMap<String, Vec<Json>>,
    ) -> String {
        let child_filed_statement = map
            .into_iter()
            .fold(String::new(), |acc, (key, mut value)| {
                format!(
                    "{}{}\n",
                    acc,
                    self.make_filed_statement_and_staking(
                        child_type_key,
                        key.as_str(),
                        //todo fix value
                        value.pop().unwrap()
                    )
                )
            });
        format!(
            "{} {}{}{}",
            self.type_statement.create_statement(child_type_key),
            self.off_side_rule.start(),
            child_filed_statement,
            self.off_side_rule.end()
        )
    }
    fn child_type_key(&self, parent_type_key: &str, child_key: &str) -> String {
        let npc = NamingPrincipalConvertor::new(child_key);
        format!("{}{}", parent_type_key, npc.to_pascal())
    }
    fn case_arr(&self, _: Vec<Json>) -> String {
        //self.mapper.make_array_type(type_str)
        todo!("case arr")
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
    //#[test]
    //fn test_get_json_from_array_json() {
    //let json = r#"
    //[
    //{
    //"name":"kai"
    //},
    //{
    //"userId":12345,
    //"test":"test-string",
    //"entities":{
    //"id":0
    //}
    //},
    //{
    //"age":20
    //}
    //]
    //"#;
    //let json = Json::from(json);
    //let  Json::Array(array) = json else {
    //panic!()
    //};
    //let mut child = BTreeMap::new();
    //child.insert("id".to_string(), Json::Number(0.into()));
    //let mut tobe = BTreeMap::new();
    //tobe.insert("name".to_string(), Json::String("kai".to_string()));
    //tobe.insert("userId".to_string(), Json::Number(12345.into()));
    //tobe.insert("test".to_string(), Json::String("test-string".to_string()));
    //tobe.insert("entities".to_string(), Json::Object(child));
    //tobe.insert("age".to_string(), Json::Number(20.into()));
    //let tobe = Json::Object(tobe);
    //assert_eq!(get_json_from_array_json(array), tobe);
    //}
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
        assert_eq!(rust.gen_from_json_(json), tobe);
    }
    #[test]
    fn test_case_rust_() {
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
