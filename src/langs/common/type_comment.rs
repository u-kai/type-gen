use std::collections::HashMap;

use crate::traits::type_statements::type_comment::TypeComment;

type TypeKey = String;
type Comment = String;
pub struct BaseTypeComment {
    comment_mark: &'static str,
    comment_map: HashMap<TypeKey, Comment>,
}

impl BaseTypeComment {
    pub fn new(comment_mark: &'static str) -> Self {
        Self {
            comment_mark,
            comment_map: HashMap::new(),
        }
    }
    pub fn add_comment(&mut self, key: &str, comment: &str) {
        if let Some(prev_comment) = self.get_comment(key) {
            let new_comment = format!("{}{}", prev_comment, self.create_comment(comment));
            self.comment_map
                .insert(key.to_string(), new_comment.to_string());
            return;
        }
        self.comment_map
            .insert(key.to_string(), self.create_comment(comment));
    }
    fn create_comment(&self, comment: &str) -> String {
        format!("{} {}\n", self.comment_mark, comment)
    }
}

impl TypeComment for BaseTypeComment {
    fn get_comment(&self, filed_key: &str) -> Option<String> {
        self.comment_map.get(filed_key).map(|s| s.clone())
    }
}

#[cfg(test)]
mod test_base_filed_comment {
    use super::*;
    #[test]
    fn add_comment() {
        let mut filed_comment = BaseTypeComment::new("//");
        filed_comment.add_comment("test", "Hello world");
        filed_comment.add_comment("test", "this is test");
        filed_comment.add_comment("name", "this is name");
        assert_eq!(
            filed_comment.get_comment("test").unwrap(),
            format!("{}\n{}\n", "// Hello world", "// this is test")
        );
        assert_eq!(
            filed_comment.get_comment("name").unwrap(),
            format!("{}\n", "// this is name")
        );
    }
}
