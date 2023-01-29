use description_generator::{
    type_description_generator::{
        DeclarePartGenerator, PropertyPartGenerator, TypeDescriptionGenerator,
    },
    type_mapper::TypeMapper,
};
use json::json::Json;
use npc::fns::to_pascal;
use rust::description_generator::{
    declare_part_generator::RustDeclarePartGenerator, mapper::RustMapper,
    property_part_generator::RustPropertyPartGenerator,
};

use crate::{
    extension::Extension,
    fileconvertor::{FileStructer, FileStructerConvertor},
};

pub type JsonToRustConvertor =
    JsonToLangConvertor<RustDeclarePartGenerator, RustPropertyPartGenerator, RustMapper>;

pub struct JsonToLangConvertor<Declear, Property, Mapper>
where
    Declear: DeclarePartGenerator<Mapper = Mapper>,
    Property: PropertyPartGenerator<Mapper>,
    Mapper: TypeMapper,
{
    dist_root: String,
    generator: TypeDescriptionGenerator<Declear, Property, Mapper>,
}
impl<Declear, Property, Mapper> JsonToLangConvertor<Declear, Property, Mapper>
where
    Declear: DeclarePartGenerator<Mapper = Mapper>,
    Property: PropertyPartGenerator<Mapper>,
    Mapper: TypeMapper,
{
    pub fn new(
        dist_root: impl Into<String>,
        generator: TypeDescriptionGenerator<Declear, Property, Mapper>,
    ) -> Self {
        Self {
            dist_root: dist_root.into(),
            generator,
        }
    }
}
impl<Declear, Property, Mapper> FileStructerConvertor
    for JsonToLangConvertor<Declear, Property, Mapper>
where
    Declear: DeclarePartGenerator<Mapper = Mapper>,
    Property: PropertyPartGenerator<Mapper>,
    Mapper: TypeMapper,
{
    fn convert(&self, filestructer: &FileStructer, extension: Extension) -> FileStructer {
        let json = Json::from(filestructer.content());
        let type_structure =
            json.into_type_structures(to_pascal(filestructer.name_without_extension()));
        let rust_type_define = self.generator.generate_concat_define(type_structure);
        filestructer.to_dist(&self.dist_root, extension, rust_type_define)
    }
}