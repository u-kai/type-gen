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

use super::{
    optional_checker::BaseOptionalChecker,
    primitive_type_statement_generator::PrimitiveTypeStatementGenerator,
};

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
            Json::Object(obj) => self.make_type_defines_from_obj(&self.root_type, obj),
        }
    }
    fn make_type_defines_from_obj(&self, type_key: &str, obj: BTreeMap<String, Json>) -> String {
        println!("type = {}", type_key);
        let (filed_statement, childrens) = obj.into_iter().fold(
            (String::new(), None),
            |(filed_statement, childrens), (filed_key, v)| match v {
                Json::Object(obj) => {
                    let child_type_key = self.child_type_key(type_key, filed_key.as_str());
                    let child_type_statement = format!(
                        "{}{}\n\n",
                        childrens.unwrap_or_default(),
                        self.make_type_defines_from_obj(&child_type_key, obj)
                    );
                    println!("key = {}", child_type_key);
                    println!("statement = {}", child_type_statement);
                    let child_type_key = if self.optional_checker.is_optional(type_key, &filed_key)
                    {
                        self.mapper.make_optional_type(&child_type_key)
                    } else {
                        child_type_key
                    };
                    let filed_statement = format!(
                        "{}{}\n",
                        filed_statement,
                        self.filed_statement
                            .create_statement(&filed_key, &child_type_key)
                    );
                    (filed_statement, Some(child_type_statement))
                }
                Json::Array(arr) => (String::new(), None), //self.case_arr_with_key(type_key, filed_key.as_str(), arr),
                _ => {
                    println!("key = {}", filed_key);
                    (
                        format!(
                            "{}{}\n",
                            filed_statement,
                            self.filed_statement.create_statement(
                                &filed_key,
                                &self.primitive_type_generaotor(type_key, &filed_key, v),
                            )
                        ),
                        None,
                    )
                }
            },
        );
        format!(
            "{} {}{}{}\n\n{}",
            self.type_statement.create_statement(type_key),
            self.off_side_rule.start(),
            filed_statement,
            self.off_side_rule.end(),
            childrens.unwrap_or_default()
        )
    }
    fn make_filed_statement_and_staking(
        &self,
        type_key: &str,
        filed_key: &str,
        obj: Json,
    ) -> String {
        let primitive_type_generaotor = PrimitiveTypeStatementGenerator::new(
            type_key,
            filed_key,
            &self.mapper,
            &self.optional_checker,
        );
        let filed_type = match obj {
            Json::String(_) => primitive_type_generaotor.case_string(),
            Json::Null => primitive_type_generaotor.case_null(),
            Json::Number(num) => primitive_type_generaotor.case_num(&num),
            Json::Boolean(_) => primitive_type_generaotor.case_boolean(),
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

    fn primitive_type_generaotor(&self, type_key: &str, filed_key: &str, json: Json) -> String {
        let primitive_type_generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            &filed_key,
            &self.mapper,
            &self.optional_checker,
        );
        match json {
            Json::String(_) => primitive_type_generator.case_string(),
            Json::Null => primitive_type_generator.case_null(),
            Json::Number(num) => primitive_type_generator.case_num(&num),
            Json::Boolean(_) => primitive_type_generator.case_boolean(),
            _ => panic!("this method is not obj or array case json -> {:?}", json),
        }
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
        let primitive_type_generaotor = PrimitiveTypeStatementGenerator::new(
            type_key,
            filed_key,
            &self.mapper,
            &self.optional_checker,
        );
        for obj in arr {
            match obj {
                Json::Object(obj) => {
                    for (k, v) in obj {
                        push_to_btree_vec(&mut map, k, v)
                    }
                }
                Json::String(_) => return primitive_type_generaotor.case_string_array(),
                Json::Null => return primitive_type_generaotor.case_null_array(),
                Json::Number(num) => return primitive_type_generaotor.case_num_array(&num),
                Json::Boolean(_) => return primitive_type_generaotor.case_boolean_array(),
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
        common::{
            filed_comment::BaseFiledComment, primitive_type_statement_generator::FakeMapper,
            type_comment::BaseTypeComment,
        },
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
    struct FakeTypeStatement {
        result: String,
    }
    impl TypeStatement for FakeTypeStatement {
        const TYPE_STATEMENT: &'static str = "";
        fn create_statement(&self, _: &str) -> String {
            self.result.clone()
        }
    }
    struct FakeFiledStatement;
    impl FiledStatement for FakeFiledStatement {
        fn create_statement(&self, filed_key: &str, filed_type: &str) -> String {
            self.add_head_space(format!(
                "{}: {}{}",
                filed_key,
                filed_type,
                Self::FILED_DERIMITA
            ))
        }
    }
    struct FakeOffSideRule;

    impl FakeOffSideRule {
        const START_AND_NEXT_LINE: &'static str = "{\n";
        const END: &'static str = "}";
    }
    impl OffSideRule for FakeOffSideRule {
        fn start(&self) -> &'static str {
            Self::START_AND_NEXT_LINE
        }
        fn end(&self) -> &'static str {
            Self::END
        }
    }
    #[test]
    fn test_make_define() {
        let json = r#"
            {
                "id":0,
                "name":"kai"
            }
        "#;
        let struct_name = "Test";
        let mut optional_checker = BaseOptionalChecker::default();
        let mapper = FakeMapper;
        let osr = FakeOffSideRule;
        optional_checker.add_require(struct_name, "id");
        let t_statement = FakeTypeStatement {
            result: format!("struct {}", struct_name),
        };
        let f_statement = FakeFiledStatement;
        let type_gen = TypeDefineGenerator::new(
            struct_name,
            mapper,
            t_statement,
            f_statement,
            osr,
            optional_checker,
        );
        let tobe = r#"struct Test {
    id: usize,
    name: Option<String>,
}

"#;
        assert_eq!(type_gen.gen_from_json_(json), tobe.to_string());
    }
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
