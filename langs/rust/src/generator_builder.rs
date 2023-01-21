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
            impl RustTypeDescriptionGeneratorBuilder {
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
    {all_comment, (comment, &str)}
    ,{pub_all,}
    ,{all_attrs,(attrs,Vec<impl Into<String>>)}
    ,{set_whitelist_with_keys,(list,Vec<impl Into<String>>)}
    ,{set_blacklist_with_keys,(list,Vec<impl Into<String>>)}
    ,{all_optional,}
);
impl_declare_part_methods!(
    {all_comment, (comment, &str)}
    ,{pub_all,}
    ,{all_attrs,(attrs,Vec<impl Into<String>>)}
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
        let builder = RustTypeDescriptionGeneratorBuilder::new();
        let generator = builder
            .declare_part_set_all_derive(vec!["Clone", "Debug"])
            .declare_part_all_comment("this is type")
            .declare_part_pub_all()
            .property_part_pub_all()
            .property_part_all_attrs(vec!["allow(unuse)"])
            .property_part_all_comment("this is property")
            .property_part_all_optional()
            .build();

        let tobe = vec![
            r#"// this is type
#[derive(Clone,Debug)]
pub struct Root {
    // this is property
    #[allow(unuse)]
    pub data: Option<Vec<RootData>>,
    // this is property
    #[allow(unuse)]
    pub id: Option<usize>,
}"#,
            r#"// this is type
#[derive(Clone,Debug)]
pub struct RootData {
    // this is property
    #[allow(unuse)]
    pub results: Option<Vec<RootDataResults>>,
}"#,
            r#"// this is type
#[derive(Clone,Debug)]
pub struct RootDataResults {
    #[serde(rename = "accountId")]
    // this is property
    #[allow(unuse)]
    pub account_id: Option<String>,
    // this is property
    #[allow(unuse)]
    pub age: Option<usize>,
    // this is property
    #[allow(unuse)]
    pub name: Option<String>,
}"#,
        ];
        let expect = generator.generate(vec![root, root_data, root_data_results]);
        for e in &expect {
            println!("{}", e);
        }
        assert_eq!(expect, tobe)
    }
}
