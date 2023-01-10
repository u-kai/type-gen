use crate::{type_description_generator::DeclarePartGenerator, type_mapper::TypeMapper};
use structure::{
    alias_type_structure::AliasTypeStructure, composite_type_structure::CompositeTypeStructure,
    parts::type_name::TypeName,
};

///
pub struct CustomizableDeclarePartGenerator<M, F1, F2>
where
    M: TypeMapper,
    F1: Fn(&str, &TypeName, String) -> String,
    F2: Fn(&str, &TypeName, String) -> String,
{
    alias_generator: CustomizableAliasTypeDeclareGenerator<M, F1>,
    composite_generator: CustomizableCompositeTypeDeclareGenerator<F2>,
}

///
impl<M, F1, F2> CustomizableDeclarePartGenerator<M, F1, F2>
where
    M: TypeMapper,
    F1: Fn(&str, &TypeName, String) -> String,
    F2: Fn(&str, &TypeName, String) -> String,
{
    pub fn new(
        alias_generator: CustomizableAliasTypeDeclareGenerator<M, F1>,
        composite_generator: CustomizableCompositeTypeDeclareGenerator<F2>,
    ) -> Self {
        Self {
            alias_generator,
            composite_generator,
        }
    }
}

///
///
impl<M, F1, F2> DeclarePartGenerator for CustomizableDeclarePartGenerator<M, F1, F2>
where
    M: TypeMapper,
    F1: Fn(&str, &TypeName, String) -> String,
    F2: Fn(&str, &TypeName, String) -> String,
{
    type Mapper = M;
    fn generate_case_alias(&self, alias_type: &AliasTypeStructure, mapper: &M) -> String {
        self.alias_generator
            .generate_type_define(&alias_type, mapper)
    }
    fn generate_case_composite(
        &self,
        composite_type: &CompositeTypeStructure,
        properties_description: String,
    ) -> String {
        self.composite_generator
            .generate_type_define(composite_type, properties_description)
    }
}

#[cfg(test)]
mod declare_part_test {
    use std::collections::BTreeMap;

    use super::*;
    #[test]
    fn test_composite_type_add_comment_and_attr() {
        let property_descriptions = "id:usize".to_string();
        let type_identify = "class";
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        struct AddAttr {}
        impl CompositeTypeDeclareConvertor for AddAttr {
            fn convert(&self, acc: &mut String, _: &CompositeTypeStructure) {
                *acc = format!("#[derive(Debug)]\n{}", acc);
            }
        }
        struct AddComment {}
        impl CompositeTypeDeclareConvertor for AddComment {
            fn convert(&self, acc: &mut String, _: &CompositeTypeStructure) {
                *acc = format!("// this is comment\n{}", acc);
            }
        }
        let add_attr = Box::new(AddAttr {});
        let add_comment = Box::new(AddComment {});
        let mut generator =
            CustomizableCompositeTypeDeclareGenerator::new(type_identify, default_concat_fn);
        generator.add_description_convertor(add_attr);
        generator.add_description_convertor(add_comment);
        let tobe = "// this is comment\n#[derive(Debug)]\nclass Test {id:usize}";
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_descriptions),
            tobe.to_string()
        );

        let type_identify = "class";
        let property_descriptions = "id:usize".to_string();
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let tobe = "class Test {id:usize}";
        let generator =
            CustomizableCompositeTypeDeclareGenerator::new(type_identify, default_concat_fn);
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_descriptions),
            tobe.to_string()
        )
    }
    #[test]
    fn test_composite_type_add_pub() {
        let property_descriptions = "id:usize".to_string();
        let type_identify = "struct";
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let tobe = "pub struct Test {id:usize}";
        struct AddPub {}
        impl TypeIdentifyConvertor for AddPub {
            fn convert(&self, acc: &mut String, _: &TypeName) {
                *acc = format!("pub {}", acc);
            }
        }
        let add_pub = Box::new(AddPub {});
        let mut generator =
            CustomizableCompositeTypeDeclareGenerator::new(type_identify, default_concat_fn);
        generator.add_type_identify_convertor(add_pub);
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_descriptions),
            tobe.to_string()
        );
    }
    #[test]
    fn test_case_composite_type_simple() {
        let property_descriptions = "id:usize".to_string();
        let type_identify = "struct";
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let tobe = "struct Test {id:usize}";
        let generator =
            CustomizableCompositeTypeDeclareGenerator::new(type_identify, default_concat_fn);
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_descriptions),
            tobe.to_string()
        );

        let type_identify = "class";
        let property_descriptions = "id:usize".to_string();
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let tobe = "class Test {id:usize}";
        let generator =
            CustomizableCompositeTypeDeclareGenerator::new(type_identify, default_concat_fn);
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_descriptions),
            tobe.to_string()
        )
    }
    #[test]
    fn test_type_define_use_convertor() {
        let property_descriptions = "    id:usize".to_string();
        let type_identify = "struct";
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        fn concat_identity_and_name_and_property_description(
            type_identify: &str,
            type_name: &TypeName,
            property_descriptions: String,
        ) -> String {
            format!(
                "{} {} {{\n{}\n}}",
                type_identify,
                type_name.as_str(),
                property_descriptions
            )
        }
        let tobe = "struct Test {
    id:usize
}";
        let generator = CustomizableCompositeTypeDeclareGenerator::new(
            type_identify,
            concat_identity_and_name_and_property_description,
        );
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_descriptions),
            tobe.to_string()
        );
    }
}

///
///
///
pub struct CustomizableCompositeTypeDeclareGenerator<F>
where
    F: Fn(&str, &TypeName, String) -> String,
{
    type_identify: &'static str,
    concat_fn: F,
    type_identify_convertors: Vec<Box<dyn TypeIdentifyConvertor>>,
    description_convertors: Vec<Box<dyn CompositeTypeDeclareConvertor>>,
}
impl<F> CustomizableCompositeTypeDeclareGenerator<F>
where
    F: Fn(&str, &TypeName, String) -> String,
{
    pub fn new(type_identify: &'static str, concat_fn: F) -> Self {
        Self {
            type_identify,
            concat_fn,
            type_identify_convertors: Vec::new(),
            description_convertors: Vec::new(),
        }
    }
    pub fn add_type_identify_convertor(&mut self, convertor: Box<dyn TypeIdentifyConvertor>) {
        self.type_identify_convertors.push(convertor);
    }
    pub fn add_description_convertor(&mut self, convertor: Box<dyn CompositeTypeDeclareConvertor>) {
        self.description_convertors.push(convertor);
    }
    pub(super) fn generate_type_define(
        &self,
        composite_type: &CompositeTypeStructure,
        properties_description: String,
    ) -> String {
        let f = &self.concat_fn;
        let type_name = composite_type.type_name();
        let type_identify = self.gen_type_identify(type_name);
        let mut description = f(&type_identify, type_name, properties_description);
        self.description_convertors
            .iter()
            .for_each(|c| c.convert(&mut description, composite_type));
        description
    }
    fn gen_type_identify(&self, type_name: &TypeName) -> String {
        let mut type_identify = self.type_identify.to_string();
        self.type_identify_convertors
            .iter()
            .for_each(|c| c.convert(&mut type_identify, type_name));
        type_identify
    }
}

pub trait TypeIdentifyConvertor {
    fn convert(&self, acc: &mut String, type_name: &TypeName) -> ();
}
pub trait CompositeTypeDeclareConvertor {
    fn convert(&self, acc: &mut String, composite_type: &CompositeTypeStructure) -> ();
}
#[cfg(test)]
mod composite_type_test {
    use std::collections::BTreeMap;

    use structure::parts::property_type::property_type_factories::make_string_type;

    use crate::type_mapper::fake_mapper::FakeTypeMapper;

    use super::*;

    #[test]
    fn test_type_description_generator_case_simple() {
        let type_identify = "struct";
        let composite_generator =
            CustomizableCompositeTypeDeclareGenerator::new(type_identify, default_concat_fn);

        let alias_type_identify = "type";
        let alias_generator =
            CustomizableAliasTypeDeclareGenerator::new(alias_type_identify, concat_fn);
        let generator = CustomizableDeclarePartGenerator::new(alias_generator, composite_generator);

        let type_name: TypeName = "Test".into();
        let alias_tobe = "type Test = String".to_string();
        let alias_type_structure = AliasTypeStructure::new(type_name.clone(), make_string_type());
        let mapper = FakeTypeMapper;
        assert_eq!(
            generator.generate_case_alias(&alias_type_structure, &mapper),
            alias_tobe.to_string()
        );

        let property_descriptions = "id:usize".to_string();
        let composite_tobe = "struct Test {id:usize}";
        let composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        assert_eq!(
            generator.generate_case_composite(&composite_type, property_descriptions),
            composite_tobe.to_string()
        );
    }
}

#[cfg(test)]
pub(super) fn default_concat_fn(
    type_identify: &str,
    type_name: &TypeName,
    property_descriptions: String,
) -> String {
    format!(
        "{} {} {{{}}}",
        type_identify,
        type_name.as_str(),
        property_descriptions
    )
}

pub trait AliasTypeIdentifyConvertor {
    fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> ();
}
pub trait AliasTypeDeclareConvertor {
    fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> ();
}
pub trait AssignmentConvertor<M: TypeMapper> {
    fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure, mapper: &M) -> ();
}
pub struct CustomizableAliasTypeDeclareGenerator<M, F>
where
    M: TypeMapper,
    F: Fn(&str, &TypeName, String) -> String,
{
    alias_type_identify: &'static str,
    concat_fn: F,
    description_convertors: Vec<Box<dyn AliasTypeDeclareConvertor>>,
    type_convertors: Vec<Box<dyn AssignmentConvertor<M>>>,
    type_identify_convertors: Vec<Box<dyn AliasTypeIdentifyConvertor>>,
}
impl<M, F> CustomizableAliasTypeDeclareGenerator<M, F>
where
    M: TypeMapper,
    F: Fn(&str, &TypeName, String) -> String,
{
    pub fn new(alias_type_identify: &'static str, concat_fn: F) -> Self {
        Self {
            alias_type_identify,
            concat_fn,
            description_convertors: Vec::new(),
            type_convertors: Vec::new(),
            type_identify_convertors: Vec::new(),
        }
    }
    pub fn add_description_convertor(&mut self, convertor: Box<dyn AliasTypeDeclareConvertor>) {
        self.description_convertors.push(convertor)
    }
    pub fn add_type_convertor(&mut self, convertor: Box<dyn AssignmentConvertor<M>>) {
        self.type_convertors.push(convertor)
    }
    pub fn add_type_identify_convertor(&mut self, convertor: Box<dyn AliasTypeIdentifyConvertor>) {
        self.type_identify_convertors.push(convertor)
    }
    pub fn generate_type_define(&self, alias_type: &AliasTypeStructure, mapper: &M) -> String {
        let f = &self.concat_fn;
        let type_identify = self.gen_type_identify(alias_type);
        let type_description = self.gen_type(alias_type, mapper);
        let mut description = f(&type_identify, &alias_type.type_name(), type_description);
        self.description_convertors
            .iter()
            .for_each(|c| c.convert(&mut description, alias_type));
        description
    }
    fn gen_type(&self, alias_type: &AliasTypeStructure, mapper: &M) -> String {
        let mut result = mapper.case_property_type(&alias_type.property_type());
        self.type_convertors
            .iter()
            .for_each(|c| c.convert(&mut result, alias_type, mapper));
        result
    }
    fn gen_type_identify(&self, alias_type: &AliasTypeStructure) -> String {
        let mut result = format!("{}", self.alias_type_identify);
        self.type_identify_convertors
            .iter()
            .for_each(|c| c.convert(&mut result, alias_type));
        result
    }
}

#[cfg(test)]
pub(super) fn concat_fn(identify: &str, type_name: &TypeName, description: String) -> String {
    format!("{} {} = {}", identify, type_name.as_str(), description)
}
#[cfg(test)]
mod test {
    use structure::parts::{
        property_type::property_type_factories::make_string_type, type_name::TypeName,
    };

    use crate::type_mapper::fake_mapper::FakeTypeMapper;

    use super::*;
    #[test]
    fn test_alias_type_case_simple() {
        let identify = "interface";
        let type_name: TypeName = "Test".into();
        let tobe = "interface Test = String".to_string();
        let mapper = FakeTypeMapper;
        let generator = CustomizableAliasTypeDeclareGenerator::new(identify, concat_fn);
        assert_eq!(
            generator.generate_type_define(
                &AliasTypeStructure::new(type_name, make_string_type()),
                &mapper
            ),
            tobe
        );
    }
    #[test]
    fn test_alias_type_case_add_pub_and_optional_condition() {
        let mapper = FakeTypeMapper;
        let alias_type_identify = "type";
        let type_name: TypeName = "Test".into();
        struct AddOptionalConvertor {
            targets: Vec<&'static str>,
        }
        impl AssignmentConvertor<FakeTypeMapper> for AddOptionalConvertor {
            fn convert(
                &self,
                acc: &mut String,
                alias_type: &AliasTypeStructure,
                m: &FakeTypeMapper,
            ) -> () {
                if self.targets.contains(&alias_type.type_name().as_str()) {
                    *acc = m.case_optional_type(acc.clone());
                }
            }
        }
        struct AddPubConvertor {
            targets: Vec<&'static str>,
        }
        impl AliasTypeIdentifyConvertor for AddPubConvertor {
            fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
                if self.targets.contains(&alias_type.type_name().as_str()) {
                    *acc = format!("pub {}", acc);
                }
            }
        }

        let mut generator =
            CustomizableAliasTypeDeclareGenerator::new(alias_type_identify, concat_fn);
        let optional_convertor = AddOptionalConvertor {
            targets: vec!["Test", "Id"],
        };
        let pub_convertor = AddPubConvertor {
            targets: vec!["Test", "Id"],
        };
        let pub_convertor = Box::new(pub_convertor);
        generator.add_type_convertor(Box::new(optional_convertor));
        generator.add_type_identify_convertor(pub_convertor);
        let tobe = "pub type Test = Option<String>".to_string();
        assert_eq!(
            generator.generate_type_define(
                &AliasTypeStructure::new(type_name, make_string_type()),
                &mapper
            ),
            tobe
        );
    }
    #[test]
    fn test_alias_type_case_add_comment_and_attr() {
        let mapper = FakeTypeMapper;
        let alias_type_identify = "type";
        let type_name: TypeName = "Test".into();
        struct AddAttrConvertor {
            targets: Vec<&'static str>,
        }
        impl AliasTypeDeclareConvertor for AddAttrConvertor {
            fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
                if self.targets.contains(&alias_type.type_name().as_str()) {
                    *acc = format!("#[derive(Debug)]\n{}", acc);
                }
            }
        }
        struct AddCommentConvertor {
            targets: Vec<&'static str>,
        }
        impl AliasTypeDeclareConvertor for AddCommentConvertor {
            fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
                if self.targets.contains(&alias_type.type_name().as_str()) {
                    *acc = format!("// this is comment\n{}", acc);
                }
            }
        }

        let mut generator =
            CustomizableAliasTypeDeclareGenerator::new(alias_type_identify, concat_fn);
        let attr_convertor = AddAttrConvertor {
            targets: vec!["Test", "Id"],
        };
        let comment_convertor = AddCommentConvertor {
            targets: vec!["Test", "Id"],
        };
        let attr_convertor = Box::new(attr_convertor);
        let comment_convertor = Box::new(comment_convertor);
        generator.add_description_convertor(attr_convertor);
        generator.add_description_convertor(comment_convertor);
        let tobe = "// this is comment\n#[derive(Debug)]\ntype Test = String".to_string();
        assert_eq!(
            generator.generate_type_define(
                &AliasTypeStructure::new(type_name, make_string_type()),
                &mapper
            ),
            tobe
        );
    }
}
