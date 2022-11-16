use std::collections::BTreeMap;

use crate::{
    json::Json,
    type_defines::statement_parts::{
        field_key::Fieldkey, field_type::FieldType, type_key::TypeKey,
    },
    utils::store_fn::push_to_btree_vec,
};

use super::lang_common::{
    field_statements::field_statement::FieldStatement, json_lang_mapper::JsonLangMapper,
    off_side_rule::OffSideRule, optional_checker::BaseOptionalChecker,
    type_statements::type_statement::TypeStatement,
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
    F: FieldStatement,
    O: OffSideRule,
{
    root_type: TypeKey,
    optional_checker: BaseOptionalChecker,
    mapper: M,
    type_statement: T,
    field_statement: F,
    off_side_rule: O,
}

impl<M, T, F, O> TypeDefineGenerator<M, T, F, O>
where
    M: JsonLangMapper,
    T: TypeStatement,
    F: FieldStatement,
    O: OffSideRule,
{
    const TYPE_DEFINE_DERIMITA: &'static str = "\n\n";
    pub fn new(
        root_type: impl Into<String>,
        mapper: M,
        type_statement: T,
        field_statement: F,
        off_side_rule: O,
        optional_checker: BaseOptionalChecker,
    ) -> Self {
        Self {
            root_type: TypeKey::new(root_type),
            optional_checker,
            mapper,
            type_statement,
            field_statement,
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
    /// array containe some type is not consider<br>
    /// example \["hello",0,{"name":"kai"}\]<br>
    /// above case is retrun Array(String)<>
    // case TestObj
    // {test : [{name:kai,age:20},{name:kai},{name:kai,age:20,like:{lang:rust,actor:hamabe}}]};
    // type_key is TestObj
    // field_key is test
    //
    fn type_defines_from_obj(
        &self,
        parent_type_key: &TypeKey,
        obj: BTreeMap<String, Json>,
    ) -> String {
        let (field_statements, childrens) = obj.into_iter().fold(
            (String::new(), None),
            |(mut field_statements, mut childrens), (field_key, json)| {
                let parent_field_key = Fieldkey::new(field_key);
                let child_type_key = TypeKey::from_parent(&parent_type_key, &parent_field_key);
                match json {
                    Json::Object(obj) => {
                        concat_optional_str(
                            &mut childrens,
                            self.type_defines_from_obj(&child_type_key, obj),
                        );
                        field_statements.push_str(
                            &self
                                .make_field_statement_case_obj(&parent_field_key, &parent_type_key),
                        );
                        (field_statements, childrens)
                    }
                    Json::Array(arr) => {
                        if arr.len() == 0 {
                            println!(
                                "{} can not define. because array is empty ",
                                self.root_type.value()
                            );
                            return (self.mapper.case_any().to_string(), None);
                        }
                        let field_statement = self.make_field_statement_case_array_json(
                            &parent_type_key,
                            &parent_field_key,
                            &arr,
                            Self::calc_array_json_nest_num(&arr),
                        );
                        let child = self.make_child_type_defines_from_array_json(
                            parent_type_key,
                            &parent_field_key,
                            arr,
                        );
                        field_statements.push_str(&field_statement);
                        concat_optional_str(&mut childrens, child.unwrap_or_default());
                        (field_statements, childrens)
                    }
                    _ => {
                        field_statements.push_str(&self.make_field_statement_case_primitive(
                            &parent_field_key,
                            parent_type_key,
                            json,
                        ));
                        (field_statements, childrens)
                    }
                }
            },
        );
        format!(
            "{} {}{}{}{}{}",
            self.type_statement.create_statement(&parent_type_key),
            self.off_side_rule.start(),
            field_statements,
            self.off_side_rule.end(),
            Self::TYPE_DEFINE_DERIMITA,
            childrens.unwrap_or_default()
        )
    }
    fn case_arr(&self, _: Vec<Json>) -> String {
        todo!("case arr")
    }
    fn make_child_type_defines_from_array_json(
        &self,
        parent_type_key: &TypeKey,
        parent_field_key: &Fieldkey,
        arr: Vec<Json>,
    ) -> Option<TypeDefine> {
        if Self::has_childres(&arr) {
            return None;
        };
        let collect_obj = self.collect_obj_from_json_array(arr);
        let obj_type_key = parent_field_key.to_type_key(parent_type_key);
        let (field_statements, childrens) = collect_obj.into_iter().fold(
            (String::new(), None),
            |(mut field_statements, mut childrens), (field_key, collected_json)| {
                let nest_num = Self::calc_collected_json_nest_num(&collected_json);
                let child_field_key = Fieldkey::new(field_key);
                let field_statement = self.make_field_statement_case_array_json(
                    &obj_type_key,
                    &child_field_key,
                    &collected_json,
                    nest_num,
                );
                field_statements.push_str(&field_statement);
                childrens = concat_optionals(
                    childrens,
                    self.make_child_type_defines_from_array_json(
                        &obj_type_key,
                        &child_field_key,
                        collected_json,
                    ),
                );
                (field_statements, childrens)
            },
        );
        let type_define_and_childrens = format!(
            "{} {}{}{}{}{}",
            self.type_statement.create_statement(&obj_type_key),
            self.off_side_rule.start(),
            field_statements,
            self.off_side_rule.end(),
            Self::TYPE_DEFINE_DERIMITA,
            childrens.unwrap_or_default()
        );
        Some(type_define_and_childrens)
    }
    fn calc_array_json_nest_num(arr: &Vec<Json>) -> usize {
        let init = 1;
        fn rec(arr: &Vec<Json>, nest_count: usize) -> usize {
            if arr.len() == 0 {
                return nest_count;
            }
            let rep = &arr[0];
            match rep {
                Json::Array(arr) => rec(arr, nest_count + 1),
                _ => nest_count,
            }
        }
        rec(arr, init)
    }
    fn calc_collected_json_nest_num(collected_json: &Vec<Json>) -> usize {
        Self::calc_array_json_nest_num(collected_json) - 1
    }
    fn has_childres(arr: &Vec<Json>) -> bool {
        if arr.len() == 0 {
            return true;
        }
        let rep = &arr[0];
        match rep {
            Json::Object(_) => false,
            Json::Array(arr) => Self::has_childres(arr),
            _ => true,
        }
    }
    fn make_field_statement_case_array_json(
        &self,
        type_key: &TypeKey,
        field_key: &Fieldkey,
        arr: &Vec<Json>,
        nest_count: usize,
    ) -> String {
        fn nest_rec(
            type_key: &TypeKey,
            field_key: &Fieldkey,
            arr: &Vec<Json>,
            nest_count: usize,
            mapper: &impl JsonLangMapper,
            optional_checker: &BaseOptionalChecker,
        ) -> FieldType {
            if arr.len() == 0 {
                return FieldType::case_nest_array_primitive(
                    type_key,
                    field_key,
                    mapper,
                    optional_checker,
                    &Json::Null,
                    nest_count,
                );
            }
            let rep = &arr[0];
            match rep {
                Json::Object(_) => FieldType::case_nest_array_obj(
                    type_key,
                    field_key,
                    nest_count,
                    mapper,
                    optional_checker,
                ),
                Json::Array(arr) => nest_rec(
                    type_key,
                    field_key,
                    arr,
                    nest_count,
                    mapper,
                    optional_checker,
                ),
                _ => FieldType::case_nest_array_primitive(
                    type_key,
                    field_key,
                    mapper,
                    optional_checker,
                    rep,
                    nest_count,
                ),
            }
        }
        self.field_statement.create_statement(
            type_key,
            field_key,
            &nest_rec(
                type_key,
                field_key,
                arr,
                nest_count,
                &self.mapper,
                &self.optional_checker,
            ),
        )
    }
    fn make_field_statement_case_obj(&self, field_key: &Fieldkey, type_key: &TypeKey) -> String {
        self.field_statement.create_statement(
            type_key,
            field_key,
            &FieldType::case_obj(type_key, field_key, &self.mapper, &self.optional_checker),
        )
    }
    fn collect_obj_from_json_array(&self, arr: Vec<Json>) -> BTreeMap<String, Vec<Json>> {
        fn rec(map: &mut BTreeMap<String, Vec<Json>>, arr: Vec<Json>) {
            for json in arr {
                match json {
                    Json::Object(obj) => {
                        for (k, v) in obj {
                            push_to_btree_vec(map, k, v);
                        }
                    }
                    Json::Array(arr) => rec(map, arr),
                    _ => {}
                }
            }
        }
        let mut map = BTreeMap::new();
        rec(&mut map, arr);
        map
    }
    fn make_field_statement_case_primitive(
        &self,
        field_key: &Fieldkey,
        type_key: &TypeKey,
        json: Json,
    ) -> String {
        self.field_statement.create_statement(
            type_key,
            field_key,
            &FieldType::case_primitive(
                type_key,
                field_key,
                &self.mapper,
                &self.optional_checker,
                json,
            ),
        )
    }
}

#[cfg(test)]
mod test_type_define_gen {

    use std::cell::RefCell;

    use crate::type_defines::generators::from_json::{
        lang_common::{
            field_statements::field_comment::BaseFieldComment,
            primitive_type_statement_generator::FakeMapper,
            type_statements::type_comment::BaseTypeComment,
        },
        rust::{
            field_statements::{
                field_attr::RustFieldAttributeStore, field_statement::RustfieldStatement,
                field_visibility::RustFieldVisibilityProvider, reserved_words::RustReservedWords,
            },
            json_lang_mapper::JsonRustMapper,
            off_side_rule::RustOffSideRule,
            rust_visibility::RustVisibility,
            type_statements::{
                type_attr::{RustTypeAttribute, RustTypeAttributeStore},
                type_statement::RustTypeStatement,
                type_visibility::RustTypeVisibilityProvider,
            },
        },
    };

    use super::*;
    struct FakeTypeStatement;

    impl TypeStatement for FakeTypeStatement {
        const TYPE_STATEMENT: &'static str = "";
        fn create_statement(&self, type_key: &TypeKey) -> String {
            format!("struct {}", type_key.value())
        }
    }
    struct FakefieldStatement;
    impl FieldStatement for FakefieldStatement {
        fn create_statement(
            &self,
            _: &TypeKey,
            field_key: &Fieldkey,
            field_type: &FieldType,
        ) -> String {
            self.add_head_space(format!(
                "{}: {}{}\n",
                field_key.original(),
                field_type.value(),
                Self::FIELD_DERIMITA
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
    ) -> TypeDefineGenerator<FakeMapper, FakeTypeStatement, FakefieldStatement, FakeOffSideRule>
    {
        let mapper = FakeMapper;
        let osr = FakeOffSideRule;
        let t_statement = FakeTypeStatement;
        let f_statement = FakefieldStatement;
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
    fn test_make_define_case_cannot_use_char_containe_field_key() {
        let struct_name = "Test";
        let json = r#"
            {
                "user:profile": {
                    "name":"kai"
                },
                "name":"kai"
            }
        "#;
        let tobe = r#"struct Test {
    name: Option<String>,
    user:profile: Option<TestUserprofile>,
}

struct TestUserprofile {
    name: Option<String>,
}

"#;
        let optional_checker = BaseOptionalChecker::default();
        assert_eq!(
            make_fake_type_generator(struct_name, optional_checker).gen_from_json(json),
            tobe
        );
    }
    #[test]
    fn test_make_define_case_nest_empty_array() {
        let struct_name = "Test";
        let json = r#"
            {
                "id":0,
                "name":"kai",
                "result":[
                    [],
                    []
                ]
            }
        "#;
        let tobe = r#"struct Test {
    id: usize,
    name: Option<String>,
    result: Option<Vec<Vec<null>>>,
}

"#;
        let mut optional_checker = BaseOptionalChecker::default();
        optional_checker.add_require(struct_name, "id");
        assert_eq!(
            make_fake_type_generator(struct_name, optional_checker).gen_from_json(json),
            tobe
        );
    }
    #[test]
    fn test_make_define_case_nest_obj_array() {
        let struct_name = "Test";
        let json = r#"
            {
                "id":0,
                "name":"kai",
                "result":[
                    [
                       {
                        "obj": [
                            {
                                "id":0
                            }
                        ]
                       } 
                    ],
                    [
                        {
                            "id":0,
                            "obj": [
                                {
                                    "name":"kai"
                                }
                            ]
                        }
                    ]
                ]
            }
        "#;
        let tobe = r#"struct Test {
    id: usize,
    name: Option<String>,
    result: Option<Vec<Vec<TestResult>>>,
}

struct TestResult {
    id: Option<usize>,
    obj: Option<Vec<TestResultObj>>,
}

struct TestResultObj {
    id: Option<usize>,
    name: Option<String>,
}

"#;
        let mut optional_checker = BaseOptionalChecker::default();
        optional_checker.add_require(struct_name, "id");
        assert_eq!(
            make_fake_type_generator(struct_name, optional_checker).gen_from_json(json),
            tobe
        );
    }
    #[test]
    fn test_make_define_case_double_primitive_array() {
        let struct_name = "Test";
        let json = r#"
            {
                "id":0,
                "name":"kai",
                "result":[
                    [
                        1
                    ],
                    [
                        2 
                    ]
                ]
            }
        "#;
        let tobe = r#"struct Test {
    id: usize,
    name: Option<String>,
    result: Option<Vec<Vec<usize>>>,
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
    fn test_make_define_case_nest_array() {
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
                            "id":0,
                            "arr":[
                                {
                                    "id":0
                                }
                            ]
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
    arr: Option<Vec<TestResultObjArr>>,
    id: Option<usize>,
    like: String,
}

struct TestResultObjArr {
    id: Option<usize>,
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
        let f_comment = BaseFieldComment::new("//");
        let f_attr = RustFieldAttributeStore::new();
        let f_visi = RustFieldVisibilityProvider::new();
        let osr = RustOffSideRule::new();
        let mapper = JsonRustMapper::new();
        let t_statement = RustTypeStatement::new(t_comment, t_visi, t_attr);
        let f_statement = RustfieldStatement::new(f_comment, RefCell::new(f_attr), f_visi, rw);
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

fn concat_optional_str(l: &mut Option<String>, r: String) {
    if let Some(l) = l {
        l.push_str(&r);
    } else {
        *l = Some(r);
    }
}

#[test]
fn test_concat_optional_str() {
    let mut s = None;
    concat_optional_str(&mut s, "hello".to_string());
    assert_eq!(s.unwrap(), "hello".to_string());
    let mut s = Some("Hello".to_string());
    concat_optional_str(&mut s, "World".to_string());
    assert_eq!(s.unwrap(), "HelloWorld".to_string())
}
fn concat_optionals(l: Option<String>, r: Option<String>) -> Option<String> {
    match (l, r) {
        (Some(mut l), Some(r)) => {
            l.push_str(&r);
            Some(l)
        }
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (None, None) => None,
    }
}
