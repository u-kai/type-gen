#[cfg(test)]
mod intergration_tests {
    use std::{fs::read_to_string, path::Path};

    use sf_df::{
        fileconvertor::{FileStructer, PathStructure},
        fileoperator::{all_file_structure, file_structures_to_files},
    };

    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn 受け取ったfilestructreの配列からディレクトリおよびファイルを生成する() {
        let root = "for-filestructure-to-file";
        let path1 = format!("{}/test.json", root);
        let path2 = format!("{}/arr.json", root);
        let path3 = format!("{}/child/child.json", root);
        let files = vec![
            FileStructer::new(r#"{"id":0}"#, PathStructure::new(&path1, "json")),
            FileStructer::new(r#"{"arr":[{"id":0}]}"#, PathStructure::new(&path2, "json")),
            FileStructer::new(
                r#"{"child":[{"id":0}]}"#,
                PathStructure::new(&path3, "json"),
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
                    PathStructure::new("./tests/jsons/test.json", "json"),
                ),
                FileStructer::new(
                    read_to_string("./tests/jsons/nests/child/array.json").unwrap(),
                    PathStructure::new("./tests/jsons/nests/child/array.json", "json"),
                ),
                FileStructer::new(
                    read_to_string("./tests/jsons/nests/child/json-placeholder.json").unwrap(),
                    PathStructure::new("./tests/jsons/nests/child/json-placeholder.json", "json"),
                ),
                FileStructer::new(
                    read_to_string("./tests/jsons/nests/test-child.json").unwrap(),
                    PathStructure::new("./tests/jsons/nests/test-child.json", "json"),
                ),
            ]
        )
    }
}
