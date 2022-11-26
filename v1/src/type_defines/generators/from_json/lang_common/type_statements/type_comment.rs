pub trait TypeComment {
    fn get_comment(&self, type_key: &str) -> Option<String>;
}

use std::collections::HashMap;

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
    fn get_comment(&self, field_key: &str) -> Option<String> {
        self.comment_map.get(field_key).map(|s| s.clone())
    }
}

#[cfg(test)]
mod test_base_field_comment {
    use super::*;
    #[test]
    fn add_comment() {
        let mut field_comment = BaseTypeComment::new("//");
        field_comment.add_comment("test", "Hello world");
        field_comment.add_comment("test", "this is test");
        field_comment.add_comment("name", "this is name");
        assert_eq!(
            field_comment.get_comment("test").unwrap(),
            format!("{}\n{}\n", "// Hello world", "// this is test")
        );
        assert_eq!(
            field_comment.get_comment("name").unwrap(),
            format!("{}\n", "// this is name")
        );
    }
}
