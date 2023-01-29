use description_generator::type_description_generator::TypeDescriptionGenerator;
use json::json::Json;
use npc::fns::to_pascal;
use rust::description_generator::{
    declare_part_generator::RustDeclarePartGenerator, mapper::RustMapper,
    property_part_generator::RustPropertyPartGenerator,
};

use crate::{
    fileconvertor::{FileStructer, FileStructerConvertor},
    filedatas::extension::Extension,
};

pub type RustTypeDescriptionGenerator =
    TypeDescriptionGenerator<RustDeclarePartGenerator, RustPropertyPartGenerator, RustMapper>;

pub struct JsonToRustConvertor {
    dist_root: String,
    generator: RustTypeDescriptionGenerator,
}

impl JsonToRustConvertor {
    pub fn new(dist_root: impl Into<String>, generator: RustTypeDescriptionGenerator) -> Self {
        Self {
            dist_root: dist_root.into(),
            generator,
        }
    }
}
impl FileStructerConvertor for JsonToRustConvertor {
    fn convert(&self, filestructer: &FileStructer, extension: Extension) -> FileStructer {
        let json = Json::from(filestructer.content());
        let type_structure =
            json.into_type_structures(to_pascal(filestructer.name_without_extension()));
        let rust_type_define = self.generator.generate_concat_define(type_structure);
        filestructer.to_dist(&self.dist_root, extension, rust_type_define)
    }
}
