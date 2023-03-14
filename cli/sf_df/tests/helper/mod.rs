use std::{fs::read_to_string, path::Path};

use sf_df::fileoperator::create_new_file;
#[cfg(test)]
pub struct TestDirectoryOperator {
    paths: Vec<String>,
}
impl TestDirectoryOperator {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }
    pub fn remove_dir_all(&self, root: &str) {
        std::fs::remove_dir_all(root).unwrap_or_default();
    }
    pub fn clean_up_before_test(&self, root: &str) {
        std::fs::remove_dir_all(root).unwrap_or_default();
    }
    pub fn prepare_file(&mut self, path: impl Into<String>, content: impl Into<String>) {
        let path = path.into();
        let content = content.into();
        create_new_file(path.clone(), content.clone());
        self.paths.push(path);
    }
    #[allow(unused)]
    pub fn assert_exist(&mut self, path: impl Into<String>) {
        let path = path.into();
        assert!(Path::new(&path).exists());
        self.paths.push(path);
    }
    pub fn assert_exist_with_content(
        &mut self,
        path: impl Into<String>,
        content: impl Into<String>,
    ) {
        let path = path.into();
        let content = content.into();
        assert!(Path::new(&path).exists());
        assert_eq!(read_to_string(&path).unwrap(), content,);
        self.paths.push(path);
    }
    #[allow(unused)]
    pub fn remove_file(&self, file_name: &str) {
        std::fs::remove_file(file_name).unwrap_or_default();
    }
    pub fn clean_up(self) {
        self.paths
            .into_iter()
            .for_each(|p| std::fs::remove_file(p).unwrap_or_default())
    }
    #[allow(unused)]
    pub fn prepare_test_json_file(&mut self, json_path: &str) {
        self.clean_up_before_test(json_path);
        self.prepare_file(
            format!("{}/test.json", json_path),
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
        self.prepare_file(
            format!("{}/nests/test-child.json", json_path),
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
        self.prepare_file(format!("{}/nests/child/json-placeholder.json",json_path), r#"
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
        self.prepare_file(
            format!("{}/nests/child/array.json", json_path),
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
    }
}
