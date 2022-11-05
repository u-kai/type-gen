use std::collections::HashMap;

use crate::traits::filed_statements::filed_comment::FiledComment;

type FiledKey = String;
type Comment = Vec<String>;
pub struct BaseFiledComment {
    comment_mark: &'static str,
    comment_map: HashMap<FiledKey, Comment>,
}

impl BaseFiledComment {
    pub fn new(comment_mark: &'static str) -> Self {
        Self {
            comment_mark,
            comment_map: HashMap::new(),
        }
    }
    pub fn add_comment(&mut self, key: &str, comment: &str) {
        if self.comment_map.contains_key(key) {
            let comment = self.create_comment(comment);
            self.comment_map
                .get_mut(key)
                .as_mut()
                .unwrap()
                .push(comment);
            return;
        }
        self.comment_map
            .insert(key.to_string(), vec![self.create_comment(comment)]);
    }
    fn create_comment(&self, comment: &str) -> String {
        format!("{} {}", self.comment_mark, comment)
    }
}

impl FiledComment for BaseFiledComment {
    fn get_comments(&self, filed_key: &str) -> Option<&Vec<String>> {
        self.comment_map.get(filed_key).map(|s| s)
    }
}

#[cfg(test)]
mod test_base_filed_comment {
    use super::*;
    #[test]
    fn add_comment() {
        let mut filed_comment = BaseFiledComment::new("//");
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
