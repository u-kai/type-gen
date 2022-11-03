use crate::type_gen::TypeGenerator;

use super::rust_struct_convertor::RustStructConvertor;

pub struct RustStructConvertorBuilder {
    struct_name: String,
    derives: Vec<String>,
    pub_structs: Vec<String>,
    pub_fileds: Vec<String>,
}
impl RustStructConvertorBuilder {
    pub fn new_with_derives(
        struct_name: impl Into<String>,
        derives: Vec<impl Into<String>>,
    ) -> Self {
        let struct_name = struct_name.into();
        let derives = derives.into_iter().map(|s| s.into()).collect();

        Self {
            struct_name,
            derives,
            pub_fileds: Vec::new(),
            pub_structs: Vec::new(),
        }
    }
    pub fn build(self) -> TypeGenerator<RustStructConvertor> {
        let convertor = RustStructConvertor::new(
            self.struct_name,
            self.derives,
            self.pub_structs,
            self.pub_fileds,
        );
        TypeGenerator::new(convertor)
    }
    pub fn set_pub_filed(mut self, filed_name: impl Into<String>) -> Self {
        self.pub_fileds.push(filed_name.into());
        self
    }
    pub fn set_pub_struct(mut self, struct_name: impl Into<String>) -> Self {
        self.pub_structs.push(struct_name.into());
        self
    }
}
