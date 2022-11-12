use std::collections::BTreeMap;

use npc::convertor::NamingPrincipalConvertor;

use crate::{
    json::Json,
    langs::rust::filed_statements::filed_statement,
    traits::{
        filed_statements::filed_statement::FiledStatement, json_lang_mapper::JsonLangMapper,
        off_side_rule::OffSideRule, optional_checker::OptionalChecker,
        type_statements::type_statement::TypeStatement,
    },
    utils::store_fn::push_to_btree_vec,
};

use super::{
    optional_checker::BaseOptionalChecker,
    primitive_type_statement_generator::PrimitiveTypeStatementGenerator,
};

pub type TypeDefine = String;
struct ChildTypeDefine {
    name: String,
    key: String,
}
impl ChildTypeDefine {
    fn new(parent_type_key: &str, key: impl Into<String>) -> Self {
        let key: String = key.into();
        let npc = NamingPrincipalConvertor::new(&key);
        let name = format!("{}{}", parent_type_key, npc.to_pascal());
        Self {
            name,
            key: key.into(),
        }
    }
    fn parent_filed_key(&self) -> &str {
        &self.key
    }
    fn type_key(&self) -> &str {
        &self.name
    }
    fn for_parent_filed(
        &self,
        parent_type_key: &str,
        optional_checker: &impl OptionalChecker,
        mapper: &impl JsonLangMapper,
    ) -> String {
        if optional_checker.is_optional(parent_type_key, &self.key) {
            mapper.make_optional_type(&self.name)
        } else {
            self.name.clone()
        }
    }
    fn for_parent_array_filed(
        &self,
        parent_type_key: &str,
        optional_checker: &impl OptionalChecker,
        mapper: &impl JsonLangMapper,
    ) -> String {
        println!("parent_key = {}", parent_type_key);
        println!("self.name = {}", self.key);
        if optional_checker.is_optional(parent_type_key, &self.key) {
            mapper.make_optional_type(&mapper.make_array_type(&self.name))
        } else {
            mapper.make_array_type(&self.name)
        }
    }
}

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
            Json::Object(obj) => self.type_defines_from_obj(&self.root_type, obj),
        }
    }
    fn primitive_array_type_generaotor(
        &self,
        type_key: &str,
        filed_key: &str,
        json: Json,
    ) -> String {
        let primitive_type_generator = PrimitiveTypeStatementGenerator::new(
            type_key,
            &filed_key,
            &self.mapper,
            &self.optional_checker,
        );
        match json {
            Json::String(_) => primitive_type_generator.case_string_array(),
            Json::Null => primitive_type_generator.case_null_array(),
            Json::Number(num) => primitive_type_generator.case_num_array(&num),
            Json::Boolean(_) => primitive_type_generator.case_boolean_array(),
            _ => panic!("this method is not obj or array case json -> {:?}", json),
        }
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
    fn case_arr(&self, _: Vec<Json>) -> String {
        //self.mapper.make_array_type(type_str)
        todo!("case arr")
    }
    /// ### array containe some type is not consider example below
    /// \["hello",0,{"name":"kai"}\]<br>
    /// above case is retrun Array(String)<>
    // case TestObj
    // {test : [{name:kai,age:20},{name:kai},{name:kai,age:20,like:{lang:rust,actor:hamabe}}]};
    // type_key is TestObj
    // filed_key is test
    //
    fn type_defines_from_obj(&self, type_key: &str, obj: BTreeMap<String, Json>) -> String {
        let (filed_statement, childrens) = obj.into_iter().fold(
            (String::new(), None),
            |(mut filed_statement, childrens), (filed_key, v)| match v {
                Json::Object(obj) => {
                    let child_type = ChildTypeDefine::new(&type_key, &filed_key);
                    let child_type_statement = format!(
                        "{}{}",
                        childrens.unwrap_or_default(),
                        self.type_defines_from_obj(child_type.type_key(), obj)
                    );
                    let filed_statement = format!(
                        "{}{}\n",
                        filed_statement,
                        self.filed_statement.create_statement(
                            child_type.parent_filed_key(),
                            &child_type.for_parent_filed(
                                type_key,
                                &self.optional_checker,
                                &self.mapper,
                            )
                        )
                    );
                    (filed_statement, Some(child_type_statement))
                }
                Json::Array(arr) => {
                    if arr.len() == 0 {
                        println!("{} can not define. because array is empty ", self.root_type);
                        return (String::new(), None);
                    }
                    let mut map = BTreeMap::new();
                    for json in arr {
                        match json {
                            Json::Object(obj) => {
                                for (k, v) in obj.clone() {
                                    push_to_btree_vec(&mut map, k, v)
                                }
                                let child_type = ChildTypeDefine::new(&type_key, &filed_key);
                                let filed_statement = format!(
                                    "{}{}\n",
                                    filed_statement,
                                    self.filed_statement.create_statement(
                                        child_type.parent_filed_key(),
                                        &child_type.for_parent_array_filed(
                                            type_key,
                                            &self.optional_checker,
                                            &self.mapper,
                                        )
                                    )
                                );
                                let child_type_statement = format!(
                                    "{}{}",
                                    childrens.unwrap_or_default(),
                                    self.type_defines_from_obj(child_type.type_key(), obj)
                                );
                                //let child_type_statement = format!(
                                //"{}{}",
                                //childrens.unwrap_or_default(),
                                //self.type_defines_from_obj(child_type.parent_filed_key(), obj)
                                //);
                                return (filed_statement, Some(child_type_statement));
                            }
                            _ => {
                                self.stack_array_filed_statement(
                                    &mut filed_statement,
                                    type_key,
                                    &filed_key,
                                    json,
                                );
                                return (filed_statement, childrens);
                            }
                        }
                    }
                    panic!()
                }
                _ => {
                    self.stack_filed_statement(&mut filed_statement, type_key, &filed_key, v);
                    (filed_statement, childrens)
                }
            },
        );
        self.create_type_statement(type_key, filed_statement, childrens.unwrap_or_default())
    }
    fn type_defines_from_arr_obj(
        &self,
        type_key: &str,
        arr: BTreeMap<String, Vec<Json>>,
    ) -> String {
        let mut filed_statement = String::new();
        let mut childrens = String::new();
        for (filed_key, v) in arr {
            for json in v {
                match json {
                    Json::Object(obj) => {
                        let data = "";
                        let data = "";
                    }
                    _ => {
                        self.stack_filed_statement(
                            &mut filed_statement,
                            type_key,
                            &filed_key,
                            json,
                        );
                        break;
                    }
                }
            }
        }
        self.create_type_statement(type_key, filed_statement, childrens)
    }
    fn stack_array_filed_statement(
        &self,
        filed_statement: &mut String,
        type_key: &str,
        filed_key: &str,
        json: Json,
    ) {
        *filed_statement += &format!(
            "{}\n",
            self.filed_statement.create_statement(
                &filed_key,
                &self.primitive_array_type_generaotor(type_key, filed_key, json),
            )
        );
    }
    fn stack_filed_statement(
        &self,
        filed_statement: &mut String,
        type_key: &str,
        filed_key: &str,
        json: Json,
    ) {
        *filed_statement += &format!(
            "{}\n",
            self.filed_statement.create_statement(
                &filed_key,
                &self.primitive_type_generaotor(type_key, filed_key, json),
            )
        );
    }
    fn create_type_statement(
        &self,
        type_key: &str,
        filed_statement: String,
        childrens: String,
    ) -> String {
        format!(
            "{} {}{}{}\n\n{}",
            self.type_statement.create_statement(type_key),
            self.off_side_rule.start(),
            filed_statement,
            self.off_side_rule.end(),
            childrens
        )
    }
}

#[cfg(test)]
mod test_type_define_gen {

    use std::cell::RefCell;

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
    struct FakeTypeStatement;

    impl TypeStatement for FakeTypeStatement {
        const TYPE_STATEMENT: &'static str = "";
        fn create_statement(&self, type_key: &str) -> String {
            format!("struct {}", type_key)
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
    fn make_fake_type_generator(
        root_key: &str,
        optional_checker: BaseOptionalChecker,
    ) -> TypeDefineGenerator<FakeMapper, FakeTypeStatement, FakeFiledStatement, FakeOffSideRule>
    {
        let mapper = FakeMapper;
        let osr = FakeOffSideRule;
        let t_statement = FakeTypeStatement;
        let f_statement = FakeFiledStatement;
        TypeDefineGenerator::new(
            root_key,
            mapper,
            t_statement,
            f_statement,
            osr,
            optional_checker,
        )
    }
    #[test]
    fn test_make_define_case_obj_and_diffarent_type_arr() {
        let struct_name = "Test";
        let json = r#"
            {
                "id":0,
                "name":"kai",
                "result":[
                    {
                        "obj": {
                            "like":"hamabe"
                        }
                    },
                    {
                        "obj": {
                            "id":0
                        }
                    },
                    {
                        "user": {
                            "id":0,
                            "name":"kai"
                        }
                    }
                ]
            }
        "#;
        let tobe = r#"struct Test {
    id: usize,
    name: Option<String>,
    result: Option<Vec<TestResult>>,
}

struct TestResult {
    obj: Option<TestResultObj>,
    user:Option<TestResultUser>
}

struct TestResultObj {
    id: usize,
    like: String,
}

struct TestResultUser {
    id: usize,
    name: String,
}

"#;
        let mut optional_checker = BaseOptionalChecker::default();
        optional_checker.add_require(struct_name, "id");
        optional_checker.add_require("TestResultObj", "like");
        assert_eq!(
            make_fake_type_generator(struct_name, optional_checker).gen_from_json(json),
            tobe
        );
    }
    #[test]
    fn test_make_define_case_obj_and_arr() {
        let struct_name = "Test";
        let json = r#"
            {
                "id":0,
                "name":"kai",
                "result":[
                    {
                        "obj": {
                            "like":"hamabe"
                        }
                    }
                ]
            }
        "#;
        let tobe = r#"struct Test {
    id: usize,
    name: Option<String>,
    result: Option<Vec<TestResult>>,
}

struct TestResult {
    obj: Option<TestResultObj>,
}

struct TestResultObj {
    like: String,
}

"#;
        let mut optional_checker = BaseOptionalChecker::default();
        optional_checker.add_require(struct_name, "id");
        optional_checker.add_require("TestResultObj", "like");
        assert_eq!(
            make_fake_type_generator(struct_name, optional_checker).gen_from_json(json),
            tobe
        );
    }
    #[test]
    fn test_make_define_case_obj() {
        let struct_name = "Test";
        let json = r#"
            {
                "id":0,
                "name":"kai",
                "obj": {
                    "like":"hamabe"
                }
            }
        "#;
        let tobe = r#"struct Test {
    id: usize,
    name: Option<String>,
    obj: Option<TestObj>,
}

struct TestObj {
    like: String,
}

"#;
        let mut optional_checker = BaseOptionalChecker::default();
        optional_checker.add_require(struct_name, "id");
        optional_checker.add_require("TestObj", "like");
        assert_eq!(
            make_fake_type_generator(struct_name, optional_checker).gen_from_json(json),
            tobe
        );
    }
    #[test]
    fn test_make_define_case_obj_has_primitive_array() {
        let json = r#"
            {
                "id":0,
                "name":"kai",
                "pri_array": ["kai","hamabe"]
            }
        "#;
        let struct_name = "Test";
        let mut optional_checker = BaseOptionalChecker::default();
        optional_checker.add_require(struct_name, "id");
        let type_gen = make_fake_type_generator(struct_name, optional_checker);
        let tobe = r#"struct Test {
    id: usize,
    name: Option<String>,
    pri_array: Option<Vec<String>>,
}

"#;
        assert_eq!(type_gen.gen_from_json(json), tobe.to_string());
    }
    #[test]
    fn test_make_define_case_only_primitive() {
        let json = r#"
            {
                "id":0,
                "name":"kai"
            }
        "#;
        let struct_name = "Test";
        let mut optional_checker = BaseOptionalChecker::default();
        optional_checker.add_require(struct_name, "id");
        let type_gen = make_fake_type_generator(struct_name, optional_checker);
        let tobe = r#"struct Test {
    id: usize,
    name: Option<String>,
}

"#;
        assert_eq!(type_gen.gen_from_json(json), tobe.to_string());
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
}

"#
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
