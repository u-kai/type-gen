use crate::{
    rust::{rust_visibility::RustVisibility, rust_visibility_provider::RustVisibilityProvider},
    traits::type_statements::type_visibility::TypeVisibility,
};

pub struct RustTypeVisibilityProvider {
    inner: RustVisibilityProvider,
}
impl RustTypeVisibilityProvider {
    pub fn new() -> Self {
        Self {
            inner: RustVisibilityProvider::new(),
        }
    }
    pub fn set_all_visibility(&mut self, visibility: RustVisibility) {
        self.inner.set_all_visibility(visibility);
    }
    pub fn add_visibility(&mut self, type_key: &str, visibility: RustVisibility) {
        self.inner.add_visibility(type_key, visibility);
    }
}

impl TypeVisibility for RustTypeVisibilityProvider {
    fn get_visibility_str(&self, type_key: &str) -> &'static str {
        self.inner.get_visibility_str(type_key)
    }
}

#[cfg(test)]
mod test_type_visibility {
    use super::*;
    #[test]
    fn get_visibility_str() {
        let mut visi = RustTypeVisibilityProvider::new();
        visi.add_visibility("test", RustVisibility::Public);
        let public: &'static str = RustVisibility::Public.into();
        let private: &'static str = RustVisibility::Private.into();
        assert_eq!(visi.get_visibility_str("test"), public);
        assert_eq!(visi.get_visibility_str("name"), private);
    }
}
