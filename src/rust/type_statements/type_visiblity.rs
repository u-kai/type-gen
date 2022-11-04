use std::collections::HashMap;

use crate::{
    rust::rust_visibility::RustVisibility, traits::type_statements::type_visibility::TypeVisibility,
};

pub struct RustTypeVisibility {
    default: RustVisibility,
    store: HashMap<String, RustVisibility>,
}
impl RustTypeVisibility {
    pub fn new() -> Self {
        Self {
            default: RustVisibility::default(),
            store: HashMap::new(),
        }
    }
    pub fn add_visibility(&mut self, type_key: &str, visibility: RustVisibility) {
        self.store.insert(type_key.to_string(), visibility);
    }
}

impl TypeVisibility for RustTypeVisibility {
    fn get_visibility_str(&self, type_key: &str) -> &'static str {
        if let Some(stored) = self.store.get(type_key).map(|v| {
            let v = *v;
            v.into()
        }) {
            return stored;
        }
        return self.default.into();
    }
}

#[cfg(test)]
mod test_type_visibility {
    use super::*;
    #[test]
    fn get_visibility_str() {
        let mut visi = RustTypeVisibility::new();
        visi.add_visibility("test", RustVisibility::Public);
        let public: &'static str = RustVisibility::Public.into();
        let private: &'static str = RustVisibility::Private.into();
        assert_eq!(visi.get_visibility_str("test"), public);
        assert_eq!(visi.get_visibility_str("name"), private);
    }
}
