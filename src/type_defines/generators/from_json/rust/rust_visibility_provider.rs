use std::collections::HashMap;

use super::rust_visibility::RustVisibility;

pub struct RustVisibilityProvider {
    all_visi: Option<RustVisibility>,
    default: RustVisibility,
    store: HashMap<String, RustVisibility>,
}
impl RustVisibilityProvider {
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
    pub fn add_visibility(&mut self, type_key: &str, visibility: RustVisibility) {
        self.store.insert(type_key.to_string(), visibility);
    }
    pub fn get_visibility_str(&self, type_key: &str) -> &'static str {
        if let Some(all) = self.all_visi {
            return all.into();
        }
        if let Some(stored) = self.store.get(type_key).map(|v| {
            let v = *v;
            v.into()
        }) {
            return stored;
        }
        return self.default.into();
    }
}
