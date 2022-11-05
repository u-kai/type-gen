use std::collections::HashMap;

use npc::{convertor::NamingPrincipalConvertor, naming_principal::NamingPrincipal};
use type_gen::{
    lang_common::{filed_comment::BaseFiledComment, type_comment::BaseTypeComment},
    rust::{
        filed_statements::{
            filed_attr::{RustFiledAttribute, RustFiledAttributeStore},
            filed_statement::RustFiledStatement,
            filed_visibilty::RustFiledVisibilityProvider,
        },
        off_side_rule::RustOffSideRule,
        reserved_words::RustReservedWords,
        type_statements::{
            type_attr::{RustTypeAttribute, RustTypeAttributeStore},
            type_statement::RustTypeStatement,
            type_visiblity::RustTypeVisibilityProvider,
        },
    },
    traits::{
        filed_statements::filed_statement::FiledStatement,
        type_statements::type_statement::TypeStatement,
    },
};

fn main() {
    let mut rust = Rust::new("Test");
    rust.add_kv("userId", "Option<String>");
    rust.add_kv("type", "Option<String>");
    rust.add_kv("id", "usize");
    let mut f_comment = BaseFiledComment::new("//");
    f_comment.add_comment("userId", "hello");
    f_comment.add_comment("userId", "world");
    let mut t_comment = BaseTypeComment::new("//");
    t_comment.add_comment("Test", "TestComment");
    let mut f_visi = RustFiledVisibilityProvider::new();
    f_visi.add_visibility(
        "type",
        type_gen::rust::rust_visibility::RustVisibility::PubilcSelf,
    );
    let mut t_visi = RustTypeVisibilityProvider::new();
    t_visi.add_visibility(
        "Test",
        type_gen::rust::rust_visibility::RustVisibility::PubilcSelf,
    );
    let mut f_attr = RustFiledAttributeStore::new();
    f_attr.set_attr(
        "id",
        RustFiledAttribute::Original("#[cfg(test)]".to_string()),
    );
    let reserved_words = RustReservedWords::new();
    let mut t_attr = RustTypeAttributeStore::new();
    t_attr.set_attr("Test", RustTypeAttribute::Derive(vec!["Debug".to_string()]));
    let osr = RustOffSideRule::new();
    let f_statement = RustFiledStatement::new();
    let t_statement = RustTypeStatement::new();
    let mut result = t_statement.create_statement("Test", &t_comment, &t_attr, &t_visi, &osr);
    for key in rust.kv.keys() {
        let new_key = if !NamingPrincipal::is_snake(&key) {
            let new_key = NamingPrincipalConvertor::new(&key).to_snake();
            f_attr.set_attr(
                new_key.as_str(),
                RustFiledAttribute::Original(format!("#[serde(rename = {})]", key)),
            );
            new_key
        } else {
            key.to_string()
        };
        result = format!(
            "{}{}\n",
            result,
            f_statement.create_statement(
                new_key.as_str(),
                &rust.kv[key],
                &f_comment,
                &f_attr,
                &f_visi,
                &reserved_words,
            )
        );
    }
    println!("{}", result);
}

struct Rust {
    struct_name: String,
    kv: HashMap<String, String>,
}

impl Rust {
    fn new(struct_name: &str) -> Self {
        Rust {
            kv: HashMap::new(),
            struct_name: String::from(struct_name),
        }
    }
    fn add_kv(&mut self, k: &str, v: &str) {
        self.kv.insert(k.to_string(), v.to_string());
    }
}
