pub trait FieldComment {
    fn get_comments(&self, field_key: &str) -> Option<&Vec<String>>;
}

use std::collections::HashMap;

use crate::utils::store_fn::push_to_kv_vec;

type FiledKey = String;
type Comment = Vec<String>;
pub struct BaseFieldComment {
    comment_mark: &'static str,
    comment_map: HashMap<FiledKey, Comment>,
}

impl BaseFieldComment {
    pub fn new(comment_mark: &'static str) -> Self {
        Self {
            comment_mark,
            comment_map: HashMap::new(),
        }
    }
    pub fn add_comment(&mut self, key: &str, comment: &str) {
        let comment = self.create_comment(comment);
        push_to_kv_vec(&mut self.comment_map, key.to_string(), comment)
    }
    fn create_comment(&self, comment: &str) -> String {
        format!("{} {}", self.comment_mark, comment)
    }
}

impl FieldComment for BaseFieldComment {
    fn get_comments(&self, filed_key: &str) -> Option<&Vec<String>> {
        self.comment_map.get(filed_key).map(|s| s)
    }
}

#[cfg(test)]
mod test_base_filed_comment {
    use super::*;
    #[test]
    fn add_comment() {
        let mut filed_comment = BaseFieldComment::new("//");
        filed_comment.add_comment("test", "Hello world");
        filed_comment.add_comment("test", "this is test");
        filed_comment.add_comment("name", "this is name");
        assert_eq!(
            filed_comment.get_comments("test").unwrap(),
            &vec!["// Hello world".to_string(), "// this is test".to_string()]
        );
        assert_eq!(
            filed_comment.get_comments("name").unwrap(),
            &vec!["// this is name".to_string()]
        );
    }
}
