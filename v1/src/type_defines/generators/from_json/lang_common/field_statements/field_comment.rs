pub trait FieldComment {
    fn get_comments(&self, type_key: &str, field_key: &str) -> Option<&Vec<String>>;
}

use std::collections::HashMap;

use crate::utils::store_fn::push_to_kv_vec;

type FiledKey = String;
type TypeKey = String;
type Comment = Vec<String>;
pub struct BaseFieldComment {
    comment_mark: &'static str,
    comment_map: HashMap<(TypeKey, FiledKey), Comment>,
}

impl BaseFieldComment {
    pub fn new(comment_mark: &'static str) -> Self {
        Self {
            comment_mark,
            comment_map: HashMap::new(),
        }
    }
    pub fn add_comment(&mut self, type_key: &str, field_key: &str, comment: &str) {
        let comment = self.create_comment(comment);
        push_to_kv_vec(
            &mut self.comment_map,
            (type_key.to_string(), field_key.to_string()),
            comment,
        )
    }
    fn create_comment(&self, comment: &str) -> String {
        format!("{} {}", self.comment_mark, comment)
    }
}

impl FieldComment for BaseFieldComment {
    fn get_comments(&self, type_key: &str, filed_key: &str) -> Option<&Vec<String>> {
        self.comment_map
            .get(&(type_key.to_string(), filed_key.to_string()))
            .map(|s| s)
    }
}

#[cfg(test)]
mod test_base_filed_comment {
    use super::*;
    #[test]
    fn add_comment() {
        let mut filed_comment = BaseFieldComment::new("//");
        filed_comment.add_comment("Test", "test", "Hello world");
        filed_comment.add_comment("Test", "test", "this is test");
        filed_comment.add_comment("Name", "name", "this is name");
        assert_eq!(
            filed_comment.get_comments("Test", "test").unwrap(),
            &vec!["// Hello world".to_string(), "// this is test".to_string()]
        );
        assert_eq!(
            filed_comment.get_comments("Name", "name").unwrap(),
            &vec!["// this is name".to_string()]
        );
    }
}
