use description_generator::type_description_generator::TypeDescriptionGenerator;

use self::{
    declare_part_generator::GoDeclarePartGenerator, mapper::GoMapper,
    property_part_generator::GoPropertyPartGenerator,
};

pub mod declare_part_generator;
pub mod mapper;
pub mod property_part_generator;
pub type GoTypeDescriptionGenerator =
    TypeDescriptionGenerator<GoDeclarePartGenerator, GoPropertyPartGenerator, GoMapper>;
