#[cfg(test)]
mod intergration_tests {
    use std::{fs::read_to_string, path::Path};

    use rust::generator_builder::RustTypeDescriptionGeneratorBuilder;
    use sf_df::{
        extension::Extension,
        fileconvertor::{FileConvetor, FileStructer, PathStructure},
        fileoperator::{all_file_structure, file_structures_to_files},
        json_to_langs::JsonToRustConvertor,
    };

    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn 受け取ったfilestructreの配列からディレクトリおよびファイルを生成する() {
        let root = "for-filestructure-to-file";
        let path1 = format!("{}/test.json", root);
        let path2 = format!("{}/arr.json", root);
        let path3 = format!("{}/child/child.json", root);
        let files = vec![
            FileStructer::new(r#"{"id":0}"#, PathStructure::new(root, &path1, "json")),
            FileStructer::new(
                r#"{"arr":[{"id":0}]}"#,
                PathStructure::new(root, &path2, "json"),
            ),
            FileStructer::new(
                r#"{"child":[{"id":0}]}"#,
                PathStructure::new(root, &path3, "json"),
            ),
        ];

        file_structures_to_files(&files);

        assert!(Path::new(&path1).exists());
        assert!(Path::new(&path2).exists(),);
        assert!(Path::new(&path3).exists(),);

        //crean up
        std::fs::remove_dir_all(root).unwrap()
    }
    #[test]
    fn exapmle配下のjsonファイル読み込んでfilestructureを生成する() {
        let files = all_file_structure("./tests/jsons", "json");
        println!(
            "{:#?}",
            files
                .iter()
                .map(|f| f.name_without_extension())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            files,
            vec![
                FileStructer::new(
                    read_to_string("./tests/jsons/test.json").unwrap(),
                    PathStructure::new("./tests/jsons", "./tests/jsons/test.json", "json"),
                ),
                FileStructer::new(
                    read_to_string("./tests/jsons/nests/child/array.json").unwrap(),
                    PathStructure::new(
                        "./tests/jsons",
                        "./tests/jsons/nests/child/array.json",
                        "json"
                    ),
                ),
                FileStructer::new(
                    read_to_string("./tests/jsons/nests/child/json-placeholder.json").unwrap(),
                    PathStructure::new(
                        "./tests/jsons",
                        "./tests/jsons/nests/child/json-placeholder.json",
                        "json"
                    ),
                ),
                FileStructer::new(
                    read_to_string("./tests/jsons/nests/test-child.json").unwrap(),
                    PathStructure::new(
                        "./tests/jsons",
                        "./tests/jsons/nests/test-child.json",
                        "json"
                    ),
                ),
            ]
        )
    }
    #[test]
    fn jsonのfilestructureをrustの型定義に変換する() {
        let source = vec![
            FileStructer::new(
                r#"{"id":0}"#,
                PathStructure::new("json", "json/test.json", "json"),
            ),
            FileStructer::new(
                r#"{"arr":[{"id":0}]}"#,
                PathStructure::new("json", "json/arr.json", "json"),
            ),
        ];
        let sut = FileConvetor::new(source);
        let generator = RustTypeDescriptionGeneratorBuilder::new().build();
        let convertor = JsonToRustConvertor::new("src", generator);
        let result = sut.convert(Extension::Rs, convertor);

        assert_eq!(
            result,
            vec![
                FileStructer::new(
                    r#"struct Test {
    id: usize,
}"#,
                    PathStructure::new("src", "src/test.rs", "rs"),
                ),
                FileStructer::new(
                    r#"struct Arr {
    arr: Vec<ArrArr>,
}
struct ArrArr {
    id: usize,
}
"#,
                    PathStructure::new("src", "src/arr.rs", "rs"),
                ),
            ]
        )
    }
}

// #[test]
//
// #[ignore]
// fn test_json_to_rust_define_gen_dist_dirs_and_dist_files() {
//     //set up config
//     let config_file = "tests/type-define-config.json";
//     let src_root = "tests/json";
//     let dist_root = "tests/dist";

//     let config = TypeDefineConfigFile::new_with_src_dist(config_file, src_root, dist_root);
//     config.write_config_file();

//     let parent = r#"
//             {
//                 "id":0,
//                 "name":"parent",
//                 "obj": {
//                     "age":20
//                 }
//             }
//         "#;
//     let child = r#"
//             {
//                 "id":0,
//                 "name":"parent",
//                 "arr": [0,1]
//             }
//         "#;
//     let grand_child = r#"
//             {
//                 "id":0,
//                 "name":"parent",
//                 "arr": [
//                     {
//                         "from":"kanagawa"
//                     }
//                 ]
//             }
//         "#;
//     let sources = SourceFiles::new(
//         src_root,
//         vec![
//             ("tests/json/parent.json", parent),
//             ("tests/json/child/child.json", child),
//             ("tests/json/child/grand_child/grand_child.json", grand_child),
//         ],
//     );
//     sources.mkdir_all_source_dirs();
//     sources.write_all_source_files();

//     json_to_rust_define(config_file);
//     let all_path = all_file_path("tests/dist")
//         .into_iter()
//         .map(|f| f.as_os_str().to_str().unwrap().to_string())
//         .collect::<Vec<_>>();
//     let tobe = vec![
//         "tests/dist/parent.rs".to_string(),
//         "tests/dist/child.rs".to_string(),
//         "tests/dist/child/grand_child.rs".to_string(),
//         "tests/dist/child/grand_child/grand_child.rs".to_string(),
//         "tests/dist/child/child.rs".to_string(),
//     ];
//     let path: &Path = "tests/dist.rs".as_ref();
//     let exist = path.exists();
//     config.clean_up();
//     sources.clean_up();
//     delete_dirs(dist_root);
//     delete_file("tests/dist.rs");
//     assert_eq!(all_path, tobe);
//     assert!(exist);
// }

// struct SourceFiles<'a> {
//     root: &'a str,
//     file: HashMap<&'a str, &'a str>,
// }
// impl<'a> SourceFiles<'a> {
//     fn new(root: &'a str, path_and_content: Vec<(&'a str, &'a str)>) -> Self {
//         let file = path_and_content.into_iter().collect();
//         SourceFiles { root, file }
//     }
//     fn clean_up(self) {
//         delete_dirs(self.root)
//     }
//     fn mkdir_all_source_dirs(&self) {
//         let paths = self.file.keys();
//         for path in paths {
//             mkdir_rec(extract_dir(path).unwrap()).unwrap()
//         }
//     }
//     fn write_all_source_files(&self) {
//         let paths = self.file.keys();
//         for path in paths {
//             let content = self.file.get(path).unwrap();
//             write_new_file(path, content)
//         }
//     }
// }
// struct TypeDefineConfigFile<'a> {
//     config_file: &'a str,
//     content: String,
// }
// impl<'a> TypeDefineConfigFile<'a> {
//     fn new(config_file: &'a str, content: &str) -> Self {
//         Self {
//             config_file,
//             content: content.to_string(),
//         }
//     }
//     fn new_with_src_dist(config_file: &'a str, src_root: &'a str, dist_root: &'a str) -> Self {
//         let content = format!(
//             r#"
//             {{
//                 "src": {{
//                     "root": "{}",
//                     "extension": "json"
//                 }},
//                 "dist": {{
//                     "root": "{}",
//                     "extension": "rs"
//                 }}
//             }}
//         "#,
//             src_root, dist_root
//         );
//         Self {
//             config_file,
//             content,
//         }
//     }
//     fn write_config_file(&self) {
//         write_new_file(self.config_file, &self.content);
//     }
//     fn clean_up(self) {
//         delete_file(self.config_file)
//     }
// }
// impl<'a> Default for TypeDefineConfigFile<'a> {
//     fn default() -> Self {
//         let content = r#"
//             {
//                 "src": {
//                     "root": "examples/jsons",
//                     "extension": "json"
//                 },
//                 "dist": {
//                     "root": "examples/dist",
//                     "extension": "rs"
//                 }
//             }
//         "#;
//         TypeDefineConfigFile::new("tests/type-define-config.json", content)
//     }
// }

// fn write_new_file<P: AsRef<Path>>(path: P, content: &str) {
//     let mut config_writer = BufWriter::new(File::create(path).unwrap());
//     config_writer.write_all(content.as_bytes()).unwrap();
// }

// fn delete_dirs<P: AsRef<Path>>(root: P) {
//     std::fs::remove_dir_all(root).unwrap()
// }
// fn delete_file<P: AsRef<Path>>(path: P) {
//     std::fs::remove_file(path).unwrap()
// }
