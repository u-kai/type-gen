mod helper;
mod integration_tests {

    use rust::generator_builder::RustTypeDescriptionGeneratorBuilder;
    use sf_df::fileconvertor::{FileStructure, PathStructure};
    use tg::{
        command::{create_rust_mod_files, json_to_rust},
        config::TypeGenSource,
    };

    use crate::helper::TestDirectoryOperator;

    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    #[test]
    fn file_structureの配列からrustのmod情報に関わるfile_structureを生成する() {
        let mut operator = TestDirectoryOperator::new();
        let root = "./rust_mod_tests";
        operator.clean_up_before_test(root);
        let source = vec![
            FileStructure::new(
                "pub type Test=String;",
                PathStructure::new("./rust_mod_tests/rusts/test.rs", "rs"),
            ),
            FileStructure::new(
                "pub type Test=String;",
                PathStructure::new("./rust_mod_tests/rusts/nests/test_child.rs", "rs"),
            ),
            FileStructure::new(
                "pub type Test=String;",
                PathStructure::new("./rust_mod_tests/rusts/nests/child/array.rs", "rs"),
            ),
            FileStructure::new(
                "pub type Test=String;",
                PathStructure::new(
                    "./rust_mod_tests/rusts/nests/child/json_placeholder.rs",
                    "rs",
                ),
            ),
        ];

        for s in &source {
            operator.prepare_file(s.path().path_str(), s.content());
        }

        create_rust_mod_files(root);

        operator.assert_exist_with_content("./rust_mod_tests.rs", "pub mod rusts;\n");
        operator.assert_exist_with_content(
            "./rust_mod_tests/rusts.rs",
            "pub mod test;\npub mod nests;\n",
        );
        operator.assert_exist_with_content(
            "./rust_mod_tests/rusts/nests.rs",
            "pub mod test_child;\npub mod child;\n",
        );
        operator.assert_exist_with_content(
            "./rust_mod_tests/rusts/nests/child.rs",
            "pub mod json_placeholder;\npub mod array;\n",
        );

        operator.remove_file("./rust_mod_tests.rs");
        operator.remove_dir_all(root);
        operator.clean_up();
    }
    #[tokio::test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    async fn jsonのファイル名を指定した場合でも型変換されたrustのファイルが出力先のディレクトリに出力される(
    ) {
        let mut json_operator = TestDirectoryOperator::new();
        json_operator.clean_up_before_test("./tests/test.json");
        json_operator.prepare_file(
            "./tests/test.json",
            r#"
            {
              "id": 0,
              "name": "kai",
              "obj": {
                "from": "kanagawa",
                "now": "????",
                "age": 20
              }
            }"#,
        );
        let mut rust_operator = TestDirectoryOperator::new();
        rust_operator.clean_up_before_test("./tests/dist");

        let generator = RustTypeDescriptionGeneratorBuilder::new()
            .declare_part_pub_all()
            .property_part_pub_all()
            .declare_part_set_all_derive_with_serde(vec!["Debug", "Clone"])
            .build();
        let src = TypeGenSource::new("./tests/test.json", "json");
        let dist = "./tests/dist";
        json_to_rust(src, dist, &generator).await;
        rust_operator.assert_exist_with_content(
            "./tests/dist/test.rs",
            r#"#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct Test {
    pub id: usize,
    pub name: String,
    pub obj: TestObj,
}
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct TestObj {
    pub age: usize,
    pub from: String,
    pub now: String,
}
"#,
        );
        rust_operator.remove_dir_all("./tests/dist");
        json_operator.remove_file("./tests/test.json");
        json_operator.remove_file("./tests/dist/dist.rs");
        rust_operator.remove_file("./tests/dist.rs");
        json_operator.clean_up();
        rust_operator.clean_up();
    }

    #[tokio::test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    async fn jsons配下のjsonファイルを一枚のrustの型定義に集約してdist配下に格納する() {
        let mut json_operator = TestDirectoryOperator::new();
        json_operator.clean_up_before_test("./tests/jsons");
        json_operator.prepare_file(
            "./tests/jsons/test.json",
            r#"
            {
              "id": 0,
              "name": "kai",
              "obj": {
                "from": "kanagawa",
                "now": "????",
                "age": 20
              }
            }"#,
        );
        json_operator.prepare_file(
            "./tests/jsons/nests/test-child.json",
            r#"
            {
              "id": 0,
              "child": [
                {
                  "hello": "world"
                }
              ]
            }
        "#,
        );
        json_operator.prepare_file("./tests/jsons/nests/child/json-placeholder.json", r#"
            [
              {
                "userId": 1,
                "id": 1,
                "title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
                "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"
              },
              {
                "userId": 1,
                "id": 2,
                "title": "qui est esse",
                "body": "est rerum tempore vitae\nsequi sint nihil reprehenderit dolor beatae ea dolores neque\nfugiat blanditiis voluptate porro vel nihil molestiae ut reiciendis\nqui aperiam non debitis possimus qui neque nisi nulla"
              }
            ] 
        "#);
        json_operator.prepare_file(
            "./tests/jsons/nests/child/array.json",
            r#"
            [
              {
                "id": 0,
                "greet": "Hello",
                "arr": [
                  {
                    "data": {
                      "id": 0
                    }
                  }
                ]
              }
            ]
        "#,
        );
        let mut rust_operator = TestDirectoryOperator::new();
        rust_operator.clean_up_before_test("./tests/dist");

        let generator = RustTypeDescriptionGeneratorBuilder::new()
            .declare_part_pub_all()
            .property_part_pub_all()
            .declare_part_set_all_derive_with_serde(vec!["Debug", "Clone"])
            .build();
        let src = TypeGenSource::new("./tests/jsons", "json");
        let dist = "./tests/dist.rs";
        json_to_rust(src, dist, &generator).await;

        rust_operator.assert_exist_with_content(
            "./tests/dist.rs",
            r#"#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct Test {
    pub id: usize,
    pub name: String,
    pub obj: TestObj,
}
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct TestObj {
    pub age: usize,
    pub from: String,
    pub now: String,
}

pub type ArrayArray = Vec<Array>;
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct Array {
    pub arr: Vec<ArrayArr>,
    pub greet: String,
    pub id: usize,
}

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct ArrayArr {
    pub data: ArrayArrData,
}

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct ArrayArrData {
    pub id: usize,
}

pub type JsonPlaceholderArray = Vec<JsonPlaceholder>;
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct JsonPlaceholder {
    pub body: String,
    pub id: usize,
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: usize,
}

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct TestChild {
    pub child: Vec<TestChildChild>,
    pub id: usize,
}
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct TestChildChild {
    pub hello: String,
}
"#,
        );
        rust_operator.remove_file("./tests/dist.rs");
        rust_operator.remove_dir_all("./tests/dist");
        json_operator.remove_dir_all("./tests/jsons");
        json_operator.clean_up();
        rust_operator.clean_up();
    }
    #[tokio::test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    async fn jsons配下のjsonファイルをrustの型定義に変換してdist配下に格納する() {
        let mut json_operator = TestDirectoryOperator::new();
        json_operator.clean_up_before_test("./tests/jsons");
        json_operator.prepare_file(
            "./tests/jsons/test.json",
            r#"
            {
              "id": 0,
              "name": "kai",
              "obj": {
                "from": "kanagawa",
                "now": "????",
                "age": 20
              }
            }"#,
        );
        json_operator.prepare_file(
            "./tests/jsons/nests/test-child.json",
            r#"
            {
              "id": 0,
              "child": [
                {
                  "hello": "world"
                }
              ]
            }
        "#,
        );
        json_operator.prepare_file("./tests/jsons/nests/child/json-placeholder.json", r#"
            [
              {
                "userId": 1,
                "id": 1,
                "title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
                "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"
              },
              {
                "userId": 1,
                "id": 2,
                "title": "qui est esse",
                "body": "est rerum tempore vitae\nsequi sint nihil reprehenderit dolor beatae ea dolores neque\nfugiat blanditiis voluptate porro vel nihil molestiae ut reiciendis\nqui aperiam non debitis possimus qui neque nisi nulla"
              }
            ] 
        "#);
        json_operator.prepare_file(
            "./tests/jsons/nests/child/array.json",
            r#"
            [
              {
                "id": 0,
                "greet": "Hello",
                "arr": [
                  {
                    "data": {
                      "id": 0
                    }
                  }
                ]
              }
            ]
        "#,
        );
        let mut rust_operator = TestDirectoryOperator::new();
        rust_operator.clean_up_before_test("./tests/dist");

        let generator = RustTypeDescriptionGeneratorBuilder::new()
            .declare_part_pub_all()
            .property_part_pub_all()
            .declare_part_set_all_derive_with_serde(vec!["Debug", "Clone"])
            .build();
        let src = TypeGenSource::new("./tests/jsons", "json");
        json_to_rust(src, "./tests/dist", &generator).await;

        rust_operator.assert_exist_with_content(
            "./tests/dist.rs",
            r#"pub mod test;
pub mod nests;
"#,
        );
        rust_operator.assert_exist_with_content(
            "./tests/dist/test.rs",
            r#"#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct Test {
    pub id: usize,
    pub name: String,
    pub obj: TestObj,
}
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct TestObj {
    pub age: usize,
    pub from: String,
    pub now: String,
}
"#,
        );
        rust_operator.assert_exist_with_content(
            "./tests/dist/nests.rs",
            r#"pub mod test_child;
pub mod child;
"#,
        );
        rust_operator.assert_exist_with_content(
            "./tests/dist/nests/test_child.rs",
            r#"#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct TestChild {
    pub child: Vec<TestChildChild>,
    pub id: usize,
}
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct TestChildChild {
    pub hello: String,
}
"#,
        );
        rust_operator.assert_exist_with_content(
            "./tests/dist/nests/child.rs",
            r#"pub mod json_placeholder;
pub mod array;
"#,
        );
        rust_operator.assert_exist_with_content(
            "./tests/dist/nests/child/array.rs",
            r#"pub type ArrayArray = Vec<Array>;
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct Array {
    pub arr: Vec<ArrayArr>,
    pub greet: String,
    pub id: usize,
}

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct ArrayArr {
    pub data: ArrayArrData,
}

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct ArrayArrData {
    pub id: usize,
}
"#,
        );
        rust_operator.assert_exist_with_content(
            "./tests/dist/nests/child/json_placeholder.rs",
            r#"pub type JsonPlaceholderArray = Vec<JsonPlaceholder>;
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct JsonPlaceholder {
    pub body: String,
    pub id: usize,
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: usize,
}
"#,
        );
        rust_operator.remove_file("./tests/dist.rs");
        rust_operator.remove_dir_all("./tests/dist");
        json_operator.remove_dir_all("./tests/jsons");
        json_operator.clean_up();
        rust_operator.clean_up();
    }
}
