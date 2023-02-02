#[cfg(test)]
mod intergration_tests {

    use std::{fs::read_to_string, path::Path};

    use rust::generator_builder::RustTypeDescriptionGeneratorBuilder;
    use sf_df::{
        extension::Extension,
        fileconvertor::{FileConvetor, FileStructer, PathStructure},
        json_to_langs::{
            create_rust_mod_file_from_filestructures, json_to_rust, JsonToRustConvertor,
        },
    };

    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn filestructureの配列からrustのmod情報に関わるfilestructureを生成する() {
        let source = vec![
            FileStructer::new(
                "pub type Test=String;",
                PathStructure::new("./tests/rusts/test.rs", "rs"),
            ),
            FileStructer::new(
                "pub type Test=String;",
                PathStructure::new("./tests/rusts/nests/test-child.rs", "rs"),
            ),
            FileStructer::new(
                "pub type Test=String;",
                PathStructure::new("./tests/rusts/nests/child/array.rs", "rs"),
            ),
            FileStructer::new(
                "pub type Test=String;",
                PathStructure::new("./tests/rusts/nests/child/rs-placeholder.rs", "rs"),
            ),
        ];
        create_rust_mod_file_from_filestructures(&source);
        assert!(Path::new("./tests/rusts.rs").exists());
        assert!(Path::new("./tests/rusts/nests.rs").exists());
        assert!(Path::new("./tests/rusts/nests/child.rs").exists(),);
    }
    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn jsons配下のjsonファイルをrustの型定義に変換してdist配下に格納する() {
        let generator = RustTypeDescriptionGeneratorBuilder::new().build();
        let dist = "./tests/dist";
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
    fn jsonのfilestructureをrustの型定義に変換する() {
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
