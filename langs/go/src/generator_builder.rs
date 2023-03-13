use description_generator::type_description_generator::TypeDescriptionGenerator;

use crate::description_generator::{
    declare_part_generator::{GoDeclarePartGenerator, GoDeclarePartGeneratorBuilder},
    mapper::GoMapper,
    property_part_generator::{GoPropertyPartGenerator, GoPropertyPartGeneratorBuilder},
};

pub struct GoTypeDescriptionGeneratorBuilder {
    pub declare_part: GoDeclarePartGeneratorBuilder,
    pub property_part: GoPropertyPartGeneratorBuilder,
}
macro_rules! impl_property_part_methods {
    ($({$method:ident, $(($key:ident, $type_:ty)),*}),*) => {
        $(
            impl GoTypeDescriptionGeneratorBuilder {
                paste::item! {
                    pub fn [<property_part_ $method>](mut self,$($key: $type_),*)-> Self {
                        self.property_part =  self.property_part.$method($($key),*);
                        self
                    }
                }
            }
        )*
    };
}
macro_rules! impl_declare_part_methods {
    ($({$method:ident, $(($key:ident, $type_:ty)),*}),*) => {
        $(
            impl GoTypeDescriptionGeneratorBuilder {
                paste::item! {
                    pub fn [<declare_part_ $method>](mut self,$($key: $type_),*)-> Self {
                        self.declare_part =  self.declare_part.$method($($key),*);
                        self
                    }
                }
            }
        )*
    };
}
impl_property_part_methods!(
    {pub_all,},
    {json_marshal,},
    {all_optional,}
    ,{set_whitelist_with_keys,(list,Vec<impl Into<String>>)}
    ,{set_blacklist_with_keys,(list,Vec<impl Into<String>>)}
);
impl_declare_part_methods!(
    {pub_all,}
);
impl GoTypeDescriptionGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            declare_part: GoDeclarePartGeneratorBuilder::new(),
            property_part: GoPropertyPartGeneratorBuilder::new(),
        }
    }
    pub fn build(
        self,
    ) -> TypeDescriptionGenerator<GoDeclarePartGenerator, GoPropertyPartGenerator, GoMapper> {
        let (d, p) = (self.declare_part.build(), self.property_part.build());
        TypeDescriptionGenerator::new(d, p, GoMapper)
    }
}
