use std::collections::BTreeMap;

use npc::convertor::NamingPrincipalConvertor;

use crate::{
    json::Json,
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
    fn make_type_key(self) -> String {
        self.name
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
        PrimitiveTypeStatementGenerator::new(
            type_key,
            &filed_key,
            &self.mapper,
            &self.optional_checker,
        )
        .from_json_to_array(json)
    }
    fn primitive_type_generaotor(&self, type_key: &str, filed_key: &str, json: Json) -> String {
        PrimitiveTypeStatementGenerator::new(
            type_key,
            &filed_key,
            &self.mapper,
            &self.optional_checker,
        )
        .from_json(json)
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
                        "{}{}",
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
                    let convertor = ArrayJsonConvertor::new(
                        type_key,
                        &filed_key,
                        arr,
                        &self.mapper,
                        &self.optional_checker,
                        &self.type_statement,
                        &self.filed_statement,
                    );
                    let (filed_, child) = convertor.filed_statement_and_childrens();
                    let child_type_statement = format!(
                        "{}{}",
                        childrens.unwrap_or_default(),
                        child.unwrap_or_default()
                    );
                    (
                        format!("{}{}", filed_statement, filed_),
                        Some(child_type_statement),
                    )
                }
                _ => {
                    self.stack_filed_statement(&mut filed_statement, type_key, &filed_key, v);
                    (filed_statement, childrens)
                }
            },
        );
        self.create_type_statement(type_key, filed_statement, childrens.unwrap_or_default())
    }
    fn stack_array_filed_statement(
        &self,
        filed_statement: &mut String,
        type_key: &str,
        filed_key: &str,
        json: Json,
    ) {
        *filed_statement += &format!(
            "{}",
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
            "{}",
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

pub(self) struct ArrayJsonConvertor<'a, M, O, T, F>
where
    M: JsonLangMapper,
    O: OptionalChecker,
    T: TypeStatement,
    F: FiledStatement,
{
    type_key: &'a str,
    filed_key: &'a str,
    arr: Vec<Json>,
    mapper: &'a M,
    optional_checker: &'a O,
    type_statement: &'a T,
    filed_statement: &'a F,
}
impl<'a, M, O, T, F> ArrayJsonConvertor<'a, M, O, T, F>
where
    M: JsonLangMapper,
    O: OptionalChecker,
    T: TypeStatement,
    F: FiledStatement,
{
    pub fn new(
        type_key: &'a str,
        filed_key: &'a str,
        arr: Vec<Json>,
        mapper: &'a M,
        optional_checker: &'a O,
        type_statement: &'a T,
        filed_statement: &'a F,
    ) -> Self {
        Self {
            type_key,
            filed_key,
            arr,
            mapper,
            optional_checker,
            type_statement,
            filed_statement,
        }
    }
    fn case_obj_filed_statement(&self) -> String {
        let type_define = ChildTypeDefine::new(self.type_key, self.filed_key);
        self.filed_statement.create_statement(
            self.filed_key,
            &type_define.for_parent_array_filed(self.type_key, self.optional_checker, self.mapper),
        )
    }
    fn case_obj_non_vec_filed_statement(&self) -> String {
        let type_define = ChildTypeDefine::new(self.type_key, self.filed_key);
        self.filed_statement.create_statement(
            self.filed_key,
            &type_define.for_parent_filed(self.type_key, self.optional_checker, self.mapper),
        )
    }
    fn this_type_key(&self) -> String {
        ChildTypeDefine::new(self.type_key, self.filed_key).make_type_key()
    }
    pub fn non_vec(self) -> (String, Option<String>) {
        let filed_statement = self.case_obj_non_vec_filed_statement();
        let type_key = self.this_type_key();
        let mut map = BTreeMap::new();
        for json in self.arr {
            match json {
                Json::Object(obj) => {
                    for (k, v) in obj {
                        push_to_btree_vec(&mut map, k, v);
                    }
                }
                _ => {
                    let p = PrimitiveTypeStatementGenerator::new(
                        self.type_key,
                        self.filed_key,
                        self.mapper,
                        self.optional_checker,
                    );
                    return (
                        self.filed_statement
                            .create_statement(self.filed_key, &p.from_json(json)),
                        None,
                    );
                }
            }
        }
        let (field_records, childrens) = map.into_iter().fold(
            (String::new(), None),
            |(filed_records, childrens), (key, array_json)| {
                let json_is_array_type = match array_json.get(0) {
                    Some(Json::Array(_)) => true,
                    _ => false,
                };
                let child = ArrayJsonConvertor::new(
                    &type_key,
                    &key,
                    array_json,
                    self.mapper,
                    self.optional_checker,
                    self.type_statement,
                    self.filed_statement,
                );
                let (filed_statement, maybe_childrens) = if json_is_array_type {
                    child.filed_statement_and_childrens()
                } else {
                    child.non_vec()
                };
                let childrens = match (childrens, maybe_childrens) {
                    (Some(acc_childrens), Some(childrens)) => {
                        Some(format!("{}{}", acc_childrens, childrens))
                    }
                    (Some(childrens), None) => Some(childrens),
                    (None, Some(childrens)) => Some(childrens),
                    (None, None) => None,
                };
                (format!("{}{}", filed_records, filed_statement), childrens)
            },
        );
        let type_define_and_childrens = format!(
            "{} {}{}{}\n\n{}",
            self.type_statement.create_statement(&type_key),
            "{\n",
            field_records,
            "}",
            childrens.unwrap_or_default()
        );

        (filed_statement, Some(type_define_and_childrens))
    }
    pub fn filed_statement_and_childrens(self) -> (String, Option<String>) {
        let filed_statement = self.case_obj_filed_statement();
        let type_key = self.this_type_key();
        let mut map = BTreeMap::new();
        for json in self.arr {
            match json {
                Json::Object(obj) => {
                    for (k, v) in obj {
                        push_to_btree_vec(&mut map, k, v);
                    }
                }
                _ => {
                    let p = PrimitiveTypeStatementGenerator::new(
                        self.type_key,
                        self.filed_key,
                        self.mapper,
                        self.optional_checker,
                    );
                    return (
                        self.filed_statement
                            .create_statement(self.filed_key, &p.from_json_to_array(json)),
                        None,
                    );
                }
            }
        }
        let (field_records, childrens) = map.into_iter().fold(
            (String::new(), None),
            |(filed_records, childrens), (key, array_json)| {
                let json_is_array_type = match array_json.get(0) {
                    Some(Json::Array(_)) => true,
                    _ => false,
                };
                let child = ArrayJsonConvertor::new(
                    &type_key,
                    &key,
                    array_json,
                    self.mapper,
                    self.optional_checker,
                    self.type_statement,
                    self.filed_statement,
                );
                let (filed_statement, maybe_childrens) = if json_is_array_type {
                    child.filed_statement_and_childrens()
                } else {
                    child.non_vec()
                };
                let childrens = match (childrens, maybe_childrens) {
                    (Some(acc_childrens), Some(childrens)) => {
                        Some(format!("{}{}", acc_childrens, childrens))
                    }
                    (Some(childrens), None) => Some(childrens),
                    (None, Some(childrens)) => Some(childrens),
                    (None, None) => None,
                };
                (format!("{}{}", filed_records, filed_statement), childrens)
            },
        );
        let type_define_and_childrens = format!(
            "{} {}{}{}\n\n{}",
            self.type_statement.create_statement(&type_key),
            "{\n",
            field_records,
            "}",
            childrens.unwrap_or_default()
        );

        (filed_statement, Some(type_define_and_childrens))
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
    #[test]
    fn test_array_json_to_type() {
        let json = r#"[
            {
                "id":0,
                "name":"kai"
            },
            {
                "id":1,
                "name":"hamabe",
                "age":22
            },
            {
                "obj":{
                    "name":"kai"
                }
            }
        ]"#;
        let json = match Json::from(json) {
            Json::Array(array) => array,
            _ => panic!(),
        };
        let mapper = FakeMapper;
        let optional_checker = BaseOptionalChecker::default();
        let type_statement = FakeTypeStatement;
        let filed_statement = FakeFiledStatement;
        let convertor = ArrayJsonConvertor::new(
            "Test",
            "obj",
            json,
            &mapper,
            &optional_checker,
            &type_statement,
            &filed_statement,
        );
        let tobe = (
            "    obj: Option<Vec<TestObj>>,
"
            .to_string(),
            Some(
                "struct TestObj {
    age: Option<usize>,
    id: Option<usize>,
    name: Option<String>,
    obj: Option<TestObjObj>,
}

struct TestObjObj {
    name: Option<String>,
}

"
                .to_string(),
            ),
        );
        assert_eq!(convertor.filed_statement_and_childrens(), tobe);
    }
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
                "{}: {}{}\n",
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
    user: Option<TestResultUser>,
}

struct TestResultObj {
    id: Option<usize>,
    like: String,
}

struct TestResultUser {
    id: Option<usize>,
    name: Option<String>,
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
