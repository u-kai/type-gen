use std::collections::HashMap;

use crate::rust::rust_visibility::RustVisibility;

pub struct RustVisibilityProvider {
    default: RustVisibility,
    store: HashMap<String, RustVisibility>,
}
impl RustVisibilityProvider {
    pub fn new() -> Self {
        Self {
            default: RustVisibility::default(),
            store: HashMap::new(),
        }
    }
    pub fn add_visibility(&mut self, type_key: &str, visibility: RustVisibility) {
        self.store.insert(type_key.to_string(), visibility);
    }
    pub fn get_visibility_str(&self, type_key: &str) -> &'static str {
        if let Some(stored) = self.store.get(type_key).map(|v| {
            let v = *v;
            v.into()
        }) {
            return stored;
        }
        return self.default.into();
    }
}
