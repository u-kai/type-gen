use std::collections::BTreeMap;

use crate::{
    json::Json,
    langs::common::optional_checker::BaseOptionalChecker,
    traits::{
        filed_statements::filed_statement::FiledStatement, json_lang_mapper::JsonLangMapper,
        off_side_rule::OffSideRule, optional_checker::OptionalChecker,
        type_statements::type_statement::TypeStatement,
    },
    utils::store_fn::push_to_btree_vec,
};

use super::{filed_key::FiledKey, filed_type::FiledType, type_key::TypeKey};

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
    root_type: TypeKey,
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
            root_type: TypeKey::new(root_type),
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
    fn type_defines_from_obj(
        &self,
        parent_type_key: &TypeKey,
        obj: BTreeMap<String, Json>,
    ) -> String {
        let (filed_statements, childrens) = obj.into_iter().fold(
            (String::new(), None),
            |(mut filed_statements, mut childrens), (filed_key, json)| {
                let parent_filed_key = FiledKey::new(filed_key);
                let child_type_key = TypeKey::from_parent(&parent_type_key, &parent_filed_key);
                match json {
                    Json::Object(obj) => {
                        childrens = Some(format!(
                            "{}{}",
                            childrens.unwrap_or_default(),
                            self.type_defines_from_obj(&child_type_key, obj)
                        ));
                        filed_statements.push_str(&self.filed_statement.create_statement(
                            &parent_filed_key,
                            &FiledType::case_obj(
                                &parent_type_key,
                                &parent_filed_key,
                                &self.mapper,
                                &self.optional_checker,
                            ),
                        ));
                        (filed_statements, childrens)
                    }
                    Json::Array(arr) => {
                        if arr.len() == 0 {
                            println!(
                                "{} can not define. because array is empty ",
                                self.root_type.value()
                            );
                            return (String::new(), None);
                        }
                        let convertor = ArrayJsonConvertor::new(
                            parent_type_key,
                            &parent_filed_key,
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
                            format!("{}{}", filed_statements, filed_),
                            Some(child_type_statement),
                        )
                    }
                    _ => {
                        filed_statements += &self.filed_statement.create_statement(
                            &parent_filed_key,
                            &FiledType::case_primitive(
                                &parent_type_key,
                                &parent_filed_key,
                                &self.mapper,
                                &self.optional_checker,
                                json,
                            ),
                        );
                        (filed_statements, childrens)
                    }
                }
            },
        );
        self.create_type_statement(
            parent_type_key,
            filed_statements,
            childrens.unwrap_or_default(),
        )
    }
    fn create_type_statement(
        &self,
        type_key: &TypeKey,
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
    type_key: &'a TypeKey,
    filed_key: &'a FiledKey,
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
        type_key: &'a TypeKey,
        filed_key: &'a FiledKey,
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
        self.filed_statement.create_statement(
            self.filed_key,
            &FiledType::case_array_obj(
                self.type_key,
                self.filed_key,
                self.mapper,
                self.optional_checker,
            ),
        )
    }
    fn case_obj_non_vec_filed_statement(&self) -> String {
        self.filed_statement.create_statement(
            self.filed_key,
            &FiledType::case_obj(
                self.type_key,
                self.filed_key,
                self.mapper,
                self.optional_checker,
            ),
        )
    }
    pub fn non_vec(self) -> (String, Option<String>) {
        let filed_statement = self.case_obj_non_vec_filed_statement();
        let type_key = self.filed_key.to_type_key(self.type_key);
        let mut map = BTreeMap::new();
        for json in self.arr {
            match json {
                Json::Object(obj) => {
                    for (k, v) in obj {
                        push_to_btree_vec(&mut map, k, v);
                    }
                }
                _ => {
                    return (
                        self.filed_statement.create_statement(
                            self.filed_key,
                            &FiledType::case_primitive(
                                self.type_key,
                                self.filed_key,
                                self.mapper,
                                self.optional_checker,
                                json,
                            ),
                        ),
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
                let filed_key = FiledKey::new(key);
                let child = ArrayJsonConvertor::new(
                    &type_key,
                    &filed_key,
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
        let type_key = self.filed_key.to_type_key(self.type_key);
        let mut map = BTreeMap::new();
        for json in self.arr {
            match json {
                Json::Object(obj) => {
                    for (k, v) in obj {
                        push_to_btree_vec(&mut map, k, v);
                    }
                }
                _ => {
                    return (
                        self.filed_statement.create_statement(
                            self.filed_key,
                            &FiledType::case_array_primitive(
                                &type_key,
                                self.filed_key,
                                self.mapper,
                                self.optional_checker,
                                json,
                            ),
                        ),
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
                let filed_key = FiledKey::new(key);
                let child = ArrayJsonConvertor::new(
                    &type_key,
                    &filed_key,
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
        let type_key = TypeKey::new("Test");
        let filed_key = FiledKey::new("obj");
        let convertor = ArrayJsonConvertor::new(
            &type_key,
            &filed_key,
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
        fn create_statement(&self, type_key: &TypeKey) -> String {
            format!("struct {}", type_key.value())
        }
    }
    struct FakeFiledStatement;
    impl FiledStatement for FakeFiledStatement {
        fn create_statement(&self, filed_key: &FiledKey, filed_type: &FiledType) -> String {
            self.add_head_space(format!(
                "{}: {}{}\n",
                filed_key.value(),
                filed_type.value(),
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
