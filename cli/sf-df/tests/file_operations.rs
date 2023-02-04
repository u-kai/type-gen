mod helper;
mod integration_tests {
    use std::{fs::read_to_string, path::Path};

    use crate::helper::TestDirectoryOperator;
    use sf_df::{
        fileconvertor::{FileStructer, PathStructure},
        fileoperator::{all_file_structure, file_structures_to_files},
    };

    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn 受け取ったfilestructreの配列からディレクトリおよびファイルを生成する() {
        let root = "for-file_structure-to-file";
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
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn example配下のjsonファイル読み込んでfile_structureを生成する() {
        let mut operator = TestDirectoryOperator::new();
        let root = "./tests-all-file-structer-tests";
        operator.prepare_test_json_file(root);
        let files = all_file_structure(root, "json");
        assert_eq!(
            files,
            vec![
                FileStructer::new(
                    read_to_string(format!("{}/test.json", root)).unwrap(),
                    PathStructure::new(format!("{}/test.json", root), "json"),
                ),
                FileStructer::new(
                    read_to_string(format!("{}/nests/child/array.json", root)).unwrap(),
                    PathStructure::new(format!("{}/nests/child/array.json", root), "json"),
                ),
                FileStructer::new(
                    read_to_string(format!("{}/nests/child/json-placeholder.json", root)).unwrap(),
                    PathStructure::new(
                        format!("{}/nests/child/json-placeholder.json", root),
                        "json"
                    ),
                ),
                FileStructer::new(
                    read_to_string(format!("{}/nests/test-child.json", root)).unwrap(),
                    PathStructure::new(format!("{}/nests/test-child.json", root), "json"),
                ),
            ]
        );
        operator.clean_up();
    }
}
