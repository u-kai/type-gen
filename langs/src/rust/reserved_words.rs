use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RustReservedWords(HashMap<&'static str, &'static str>);
impl RustReservedWords {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("type", "r#type");
        map.insert("ref", "r#ref");
        map.insert("match", "r#match");
        map.insert("use", "r#use");
        map.insert("as", "r#as");
        map.insert("if", "r#if");
        map.insert("override", "r#override");
        map.insert("virtual", "r#virtual");
        map.insert("while", "r#while");
        map.insert("super", "r#super");
        map.insert("crate", "r#crate");
        map.insert("abstract", "r#abstract");
        map.insert("static", "r#static");
        map.insert("typeof", "r#typeof");
        map.insert("mod", "r#mod");
        map.insert("self", "r#self");
        map.insert("Self", "r#Self");
        map.insert("extern", "r#extern");
        map.insert("f64", "r#f64");
        map.insert("i64", "r#i64");
        map.insert("u64", "r#u64");
        Self(map)
    }
    pub fn get_or_origin<'a>(&'a self, key: &'a str) -> &'a str {
        match self.0.get(key) {
            Some(reseved) => reseved,
            None => key,
        }
    }
}
