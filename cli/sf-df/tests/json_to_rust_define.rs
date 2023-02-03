mod helper;
mod integration_tests {

    use std::path::Path;

    use rust::generator_builder::RustTypeDescriptionGeneratorBuilder;
    use sf_df::{
        extension::Extension,
        fileconvertor::{FileConvetor, FileStructer, PathStructure},
        json_to_langs::{create_rust_mod_files, json_to_rust, JsonToRustConvertor},
    };

    use crate::helper::TestDirectoryOperator;

    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn file_structureの配列からrustのmod情報に関わるfile_structureを生成する() {
        let mut operator = TestDirectoryOperator::new();
        let root = "./rust_mod_tests";
        operator.clean_up_before_test(root);
        let source = vec![
            FileStructer::new(
                "pub type Test=String;",
                PathStructure::new("./rust_mod_tests/rusts/test.rs", "rs"),
            ),
            FileStructer::new(
                "pub type Test=String;",
                PathStructure::new("./rust_mod_tests/rusts/nests/test_child.rs", "rs"),
            ),
            FileStructer::new(
                "pub type Test=String;",
                PathStructure::new("./rust_mod_tests/rusts/nests/child/array.rs", "rs"),
            ),
            FileStructer::new(
                "pub type Test=String;",
                PathStructure::new("./rust_mod_tests/rusts/nests/child/rs_placeholder.rs", "rs"),
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
            "pub mod rs_placeholder;\npub mod array;\n",
        );

        operator.clean_up_before_test(root);
        std::fs::remove_file("./rust_mod_tests.rs").unwrap();
    }
    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn jsons配下のjsonファイルをrustの型定義に変換してdist配下に格納する() {
        let generator = RustTypeDescriptionGeneratorBuilder::new().build();
        let dist = "./tests/dist";
        //crean up
        std::fs::remove_dir_all("./tm").unwrap();
        json_to_rust("./tests/jsons", dist, generator);

        assert!(Path::new("./tests/dist/test.rs").exists());
        //assert!(Path::new("./tests/dist/nests.rs").exists());
        assert!(Path::new("./tests/dist/nests/test_child.rs").exists(),);
        //assert!(Path::new("./tests/dist/nests/child.rs").exists(),);
        assert!(Path::new("./tests/dist/nests/child/array.rs").exists(),);
        assert!(Path::new("./tests/dist/nests/child/json_placeholder.rs").exists(),);

        //crean up
        std::fs::remove_dir_all(dist).unwrap()
        // assert_eq!(read_to_string("./tests/dist/test.rs").unwrap(),
        // r#""#
        // );
        // //assert_eq!(read_to_string("./tests/dist/nests.rs").unwrap());
        // assert_eq!(read_to_string("./tests/dist/nests/test_child.rs").unwrap(),);
        // //assert_eq!(read_to_string("./tests/dist/nests/child.rs").unwrap(),);
        // assert_eq!(read_to_string("./tests/dist/nests/child/array.rs").unwrap(),);
    }
    #[test]
    fn jsonのfile_structureをrustの型定義に変換する() {
        let source = vec![
            FileStructer::new(r#"{"id":0}"#, PathStructure::new("json/test.json", "json")),
            FileStructer::new(
                r#"{"arr":[{"id":0}]}"#,
                PathStructure::new("json/arr.json", "json"),
            ),
        ];
        let sut = FileConvetor::new(source);
        let generator = RustTypeDescriptionGeneratorBuilder::new().build();
        let convertor = JsonToRustConvertor::new("json", generator);
        let result = sut.convert("src", Extension::Rs, convertor);

        assert_eq!(
            result,
            vec![
                FileStructer::new(
                    r#"struct Test {
    id: usize,
}"#,
                    PathStructure::new("src/test.rs", "rs"),
                ),
                FileStructer::new(
                    r#"struct Arr {
    arr: Vec<ArrArr>,
}
struct ArrArr {
    id: usize,
}
"#,
                    PathStructure::new("src/arr.rs", "rs"),
                ),
            ]
        )
    }
}
