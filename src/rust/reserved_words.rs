use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct ReservedWords(Rc<HashMap<&'static str, &'static str>>);
impl ReservedWords {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("type", "r#type");
        map.insert("use", "r#use");
        map.insert("as", "r#as");
        map.insert("if", "r#if");
        map.insert("while", "r#while");
        map.insert("super", "r#super");
        map.insert("crate", "r#crate");
        map.insert("abstract", "r#abstruct");
        map.insert("typeof", "r#typeof");
        map.insert("mod", "r#mod");
        map.insert("self", "r#self");
        map.insert("Self", "r#Self");
        map.insert("extern", "r#extern");
        map.insert("f64", "r#f64");
        Self(Rc::new(map))
    }
    pub fn get_or_default(&self, key: &str) -> String {
        match self.0.get(key) {
            Some(reseved) => reseved.to_string(),
            None => key.to_string(),
        }
    }
    pub fn clone(&self) -> Self {
        ReservedWords(self.0.clone())
    }
}
