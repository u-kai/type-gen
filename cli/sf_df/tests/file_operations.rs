mod helper;
mod integration_tests {
    use std::fs::read_to_string;

    use crate::helper::TestDirectoryOperator;
    use sf_df::{
        fileconvertor::{FileStructer, PathStructure},
        fileoperator::{
            add_to_file, all_file_path, all_file_structure, create_new_file,
            file_structures_to_files, mkdir_rec,
        },
    };
    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn for_testディレクトリ内の全てのファイルから指定した拡張子だけfile_structureとして生成する() {
        // this test context is exist test directory
        let mut operator = TestDirectoryOperator::new();

        operator.clean_up_before_test("./for-test");
        operator.prepare_file("./for-test/rust.rs", "fn main(){}");
        operator.prepare_file("./for-test/child/rust_child.rs", "fn main2(){}");

        let tobe = vec![
            FileStructer::new(
                "fn main(){}",
                PathStructure::new("./for-test/rust.rs", "rs"),
            ),
            FileStructer::new(
                "fn main2(){}",
                PathStructure::new("./for-test/child/rust_child.rs", "rs"),
            ),
        ];
        assert_eq!(all_file_structure("./for-test", "rs"), tobe);
        operator.remove_dir_all("./for-test");
    }
    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn for_testディレクトリ内の全てのファイルのパスを取得する() {
        // this test context is exist test directory
        let mut operator = TestDirectoryOperator::new();

        operator.clean_up_before_test("./for-test");
        operator.prepare_file("./for-test/parent.txt", "");
        operator.prepare_file("./for-test/rust.rs", "");
        operator.prepare_file("./for-test/child/child.txt", "");
        operator.prepare_file("./for-test/child/grand_child/grand_child.txt", "");
        operator.prepare_file("./for-test/child/rust_child.rs", "");
        let tobe = vec![
            "./for-test/parent.txt".to_string(),
            "./for-test/rust.rs".to_string(),
            "./for-test/child/child.txt".to_string(),
            "./for-test/child/grand_child/grand_child.txt".to_string(),
            "./for-test/child/rust_child.rs".to_string(),
        ];
        assert_eq!(
            all_file_path("./for-test")
                .into_iter()
                .map(|p| p.to_str().unwrap().to_string())
                .collect::<Vec<_>>(),
            tobe
        );
        operator.remove_dir_all("./for-test");
    }
    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn 存在しない指定されたディレクトリを再起的に生成する() {
        let path = "./mkdir/mkdir_rec/mkdir_rec_child";
        let _sut = mkdir_rec(path).unwrap();
        let mut operator = TestDirectoryOperator::new();
        operator.assert_exist("mkdir/");
        operator.assert_exist("mkdir/mkdir_rec/");
        operator.assert_exist("mkdir/mkdir_rec/mkdir_rec_child");

        operator.remove_dir_all("mkdir");
        operator.clean_up();
    }
    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn 指定されたファイルパスを存在しないディレクトリも含めて作成する() {
        let new_path = "not-exist/non-exist/new-file.txt";
        let content = "test hello world";

        create_new_file(new_path, content);

        let mut operator = TestDirectoryOperator::new();
        operator.assert_exist("not-exist");
        operator.assert_exist("not-exist/non-exist");
        operator.assert_exist_with_content("not-exist/non-exist/new-file.txt", content);

        operator.remove_dir_all("mkdir");
        operator.remove_dir_all("not-exist");
        operator.clean_up();
    }
    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn 元々存在しているファイルに対して追記する() {
        let path = "./tests-add_to_file/test.txt";
        let init_content = "hello";
        let mut operator = TestDirectoryOperator::new();
        operator.clean_up_before_test("./tests-add_to_file");
        operator.prepare_file(path, init_content);

        add_to_file(path, " world");

        operator.assert_exist_with_content(path, "hello world");

        operator.remove_dir_all("./tests-add_to_file");
        operator.clean_up();
    }

    #[test]
    #[ignore = "watchでテストする際にwatchが生成のたびにループしてしまうので"]
    fn 受け取ったfilestructreの配列からディレクトリおよびファイルを生成する() {
        let mut operator = TestDirectoryOperator::new();
        let root = "for_file_structure_to_file";
        operator.clean_up_before_test(root);
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

        file_structures_to_files(files, sf_df::fileoperator::NamingPrincipal::Snake);
        operator.assert_exist_with_content(&path1, r#"{"id":0}"#);
        operator.assert_exist_with_content(&path2, r#"{"arr":[{"id":0}]}"#);
        operator.assert_exist_with_content(&path3, r#"{"child":[{"id":0}]}"#);

        operator.remove_dir_all(root);
        operator.clean_up();
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
        operator.remove_dir_all(root);
        operator.clean_up();
    }
}
