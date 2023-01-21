use description_generator::{
    customizable::{
        declare_part_generator::{
            CustomizableAliasTypeDeclareGenerator, CustomizableCompositeTypeDeclareGenerator,
        },
        property_part_generator::CustomizablePropertyDescriptionGenerator,
    },
    type_description_generator::TypeDescriptionGenerator,
};
use structure::parts::type_name::TypeName;

use crate::description_generator::{
    declare_part_generator::{RustDeclarePartGenerator, RustDeclarePartGeneratorBuilder},
    mapper::RustMapper,
    property_part_generator::{RustPropertyPartGenerator, RustPropertyPartGeneratorBuilder},
};

pub struct RustTypeDescriptionGeneratorBuilder {
    pub declare_part: RustDeclarePartGeneratorBuilder,
    pub property_part: RustPropertyPartGeneratorBuilder,
}
macro_rules! impl_property_part_methods {
    ($({$method:ident, $(($key:ident, $type_:ty)),*}),*) => {
        $(
            impl RustTypeDescriptionGeneratorBuilder {
                pub fn $method(mut self,$($key: $type_),*)-> Self {
                    self.proerty_part =  self.proerty_part.$method($($key),*);
                    self
                }
            }
        )*
    };
}
macro_rules! impl_declare_part_methods {
    ($({$method:ident, $(($key:ident, $type_:ty)),*}),*) => {
        $(
            impl RustTypeDescriptionGeneratorBuilder {
                pub fn $method(mut self,$($key: $type_),*)-> Self {
                    self.declare_part =  self.declare_part.$method($($key),*);
                    self
                }
            }
        )*
    };
}
// impl_property_part_methods!(
//     {all_comment, (comment, &str)}
//     ,{pub_all,}
//     ,{set_all_derive,(derives,Vec<impl Into<String>>)}
//     ,{set_whitelist,(list,Vec<impl Into<String>>)}
//     ,{set_blacklist,(list,Vec<impl Into<String>>)}
// );
impl_declare_part_methods!(
    {all_comment, (comment, &str)}
    ,{pub_all,}
    ,{set_all_derive,(derives,Vec<impl Into<String>>)}
    ,{set_whitelist,(list,Vec<impl Into<String>>)}
    ,{set_blacklist,(list,Vec<impl Into<String>>)}
);
impl RustTypeDescriptionGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            declare_part: RustDeclarePartGeneratorBuilder::new(),
            property_part: RustPropertyPartGeneratorBuilder::new(),
        }
    }
    pub fn build(
        self,
    ) -> TypeDescriptionGenerator<RustDeclarePartGenerator, RustPropertyPartGenerator, RustMapper>
    {
        let (d, p) = (self.declare_part.build(), self.property_part.build());
        TypeDescriptionGenerator::new(d, p, RustMapper)
    }
    pub fn change_property_generator(
        &mut self,
    ) -> &mut CustomizablePropertyDescriptionGenerator<fn(String, String) -> String, RustMapper>
    {
        self.property_part.change_property_generator()
    }
    pub fn change_composite_generator(
        &mut self,
    ) -> &mut CustomizableCompositeTypeDeclareGenerator<fn(&str, &TypeName, String) -> String> {
        self.declare_part.change_composite_generator()
    }
    pub fn change_alias_generator(
        &mut self,
    ) -> &mut CustomizableAliasTypeDeclareGenerator<RustMapper, fn(&str, &TypeName, String) -> String>
    {
        self.declare_part.change_alias_generator()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use structure::{
        parts::property_type::property_type_factories::{
            make_array_type, make_custom_type, make_string_type, make_usize_type,
        },
        type_structure::TypeStructure,
    };
    #[test]
    fn integration_test_case_set_all() {
        let root = TypeStructure::make_composite(
            "Root",
            vec![
                ("id", make_usize_type()),
                ("data", make_array_type(make_custom_type("RootData"))),
            ],
        );
        let root_data = TypeStructure::make_composite(
            "RootData",
            vec![(
                "results",
                make_array_type(make_custom_type("RootDataResults")),
            )],
        );
        let root_data_results = TypeStructure::make_composite(
            "RootDataResults",
            vec![
                ("name", make_string_type()),
                ("age", make_usize_type()),
                ("accountId", make_string_type()),
            ],
        );
        let mut builder = RustTypeDescriptionGeneratorBuilder::new();
        //builder.all_comment("this is type").
        //     let builder = builder
        //         .declare_part
        //         .all_comment("this is type")
        //         .set_all_derive(vec!["Clone", "Debug"])
        //         .pub_all();
        // }
    }
}
