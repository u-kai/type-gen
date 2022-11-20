use std::collections::HashMap;

use crate::type_defines::generators::from_json::{
    lang_common::field_statements::field_visibility::FieldVisibility,
    rust::rust_visibility::RustVisibility,
};

pub struct RustFieldVisibilityProvider {
    all_visi: Option<RustVisibility>,
    default: RustVisibility,
    store: HashMap<(String, String), RustVisibility>,
}
impl RustFieldVisibilityProvider {
    pub fn new() -> Self {
        Self {
            all_visi: None,
            default: RustVisibility::default(),
            store: HashMap::new(),
        }
    }
    pub fn set_all_visibility(&mut self, visibility: RustVisibility) {
        self.all_visi = Some(visibility);
    }
    pub fn add_visibility(&mut self, type_key: &str, field_key: &str, visibility: RustVisibility) {
        self.store
            .insert((type_key.to_string(), field_key.to_string()), visibility);
    }
}

impl FieldVisibility for RustFieldVisibilityProvider {
    fn get_visibility_str(&self, type_key: &str, field_key: &str) -> &'static str {
        if let Some(all) = self.all_visi {
            return all.into();
        }
        if let Some(stored) = self
            .store
            .get(&(type_key.to_string(), field_key.to_string()))
            .map(|v| {
                let v = *v;
                v.into()
            })
        {
            return stored;
        }
        self.default.into()
    }
}

#[cfg(test)]
mod test_type_visibility {
    use super::*;
    #[test]
    fn get_visibility_str() {
        let mut visi = RustFieldVisibilityProvider::new();
        visi.add_visibility("Test", "test", RustVisibility::Public);
        let public: &'static str = RustVisibility::Public.into();
        let private: &'static str = RustVisibility::Private.into();
        assert_eq!(visi.get_visibility_str("Test", "test"), public);
        assert_eq!(visi.get_visibility_str("Name", "name"), private);
    }
}
