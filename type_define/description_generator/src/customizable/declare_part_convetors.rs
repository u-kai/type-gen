pub struct AddHeaderConvertor<'a> {
    header: &'a str,
    store: Vec<&'a str>,
}
impl<'a> AddHeaderConvertor<'a> {
    pub fn new(header: &'a str) -> Self {
        Self {
            header,
            store: Vec::new(),
        }
    }
    pub fn add(&mut self, type_name: impl Into<&'a str>) {
        self.store.push(type_name.into());
    }
    fn containe_list(&self, type_name: &'a str) -> bool {
        self.store.contains(&type_name)
    }
}
pub struct BlackListConvertor<'a> {
    store: Vec<&'a str>,
}

impl<'a> BlackListConvertor<'a> {
    pub fn new() -> Self {
        Self { store: Vec::new() }
    }
    pub fn add(&mut self, type_name: &'a str) {
        self.store.push(type_name)
    }
    fn containe_list(&self, type_name: &'a str) -> bool {
        self.store.contains(&type_name)
    }
}

pub struct WhiteListConvertor<'a> {
    store: Vec<&'a str>,
}

impl<'a> WhiteListConvertor<'a> {
    pub fn new() -> Self {
        Self { store: Vec::new() }
    }
    pub fn add(&mut self, type_name: &'a str) {
        self.store.push(type_name)
    }
    fn containe_list(&self, type_name: &'a str) -> bool {
        self.store.contains(&type_name)
    }
}

pub mod composite_type {
    use super::*;
    use crate::customizable::declare_part_generator::CompositeTypeDeclareConvertor;

    impl<'a> CompositeTypeDeclareConvertor for BlackListConvertor<'a> {
        fn convert(
            &self,
            acc: &mut String,
            composite_type: &structure::composite_type_structure::CompositeTypeStructure,
        ) -> () {
            if self.containe_list(composite_type.type_name().as_str()) {
                *acc = String::new()
            }
        }
    }

    impl<'a> CompositeTypeDeclareConvertor for WhiteListConvertor<'a> {
        fn convert(
            &self,
            acc: &mut String,
            composite_type: &structure::composite_type_structure::CompositeTypeStructure,
        ) -> () {
            if !self.containe_list(composite_type.type_name().as_str()) {
                *acc = String::new()
            }
        }
    }
}
pub mod alias_type {
    use super::{AddHeaderConvertor, BlackListConvertor, WhiteListConvertor};
    use crate::customizable::declare_part_generator::{
        AliasTypeDeclareConvertor, AliasTypeIdentifyConvertor,
    };
    use structure::alias_type_structure::AliasTypeStructure;

    impl<'a> AliasTypeIdentifyConvertor for AddHeaderConvertor<'a> {
        fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
            if self.containe_list(alias_type.type_name().as_str()) {
                *acc = format!("{}{}", self.header, acc)
            }
        }
    }
    impl<'a> AliasTypeDeclareConvertor for BlackListConvertor<'a> {
        fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
            if self.containe_list(alias_type.type_name().as_str()) {
                *acc = String::new()
            }
        }
    }

    impl<'a> AliasTypeDeclareConvertor for WhiteListConvertor<'a> {
        fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
            if !self.containe_list(alias_type.type_name().as_str()) {
                *acc = String::new()
            }
        }
    }
}
#[cfg(test)]
mod composite_case_test {
    use crate::customizable::declare_part_generator::CompositeTypeDeclareConvertor;

    use super::*;
    use std::collections::BTreeMap;
    use structure::composite_type_structure::CompositeTypeStructure;
    #[test]
    fn test_black_list_convertor_case_containe() {
        let name = "Test";
        let mut acc = String::from("type Test = String;");
        let tobe = String::new();
        let mut black_list = BlackListConvertor::new();
        black_list.add(name);
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        black_list.convert(&mut acc, &dummy_composite_type);
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_black_list_convertor_case_not_containe() {
        let name = "Test";
        let mut acc = String::from("type Test = String;");
        let tobe = acc.clone();
        let black_list = BlackListConvertor::new();
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        black_list.convert(&mut acc, &dummy_composite_type);
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_white_list_convertor_case_containe() {
        let name = "Test";
        let mut acc = String::from("type Test = String;");
        let tobe = acc.clone();
        let mut white_list = WhiteListConvertor::new();
        white_list.add(name);
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        white_list.convert(&mut acc, &dummy_composite_type);
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_white_list_convertor_case_not_containe() {
        let name = "Test";
        let mut acc = String::from("type Test = String;");
        let tobe = String::new();
        let white_list = WhiteListConvertor::new();
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        white_list.convert(&mut acc, &dummy_composite_type);
        assert_eq!(acc, tobe);
    }
}

#[cfg(test)]
mod alias_case_test {
    use crate::customizable::declare_part_generator::{
        AliasTypeDeclareConvertor, AliasTypeIdentifyConvertor,
    };

    use super::*;
    use structure::{
        alias_type_structure::AliasTypeStructure,
        parts::property_type::property_type_factories::make_string_type,
    };
    #[test]
    fn test_add_header_case_containe() {
        let mut acc = String::from("type Test = String;");
        let tobe = format!("pub {}", acc);
        let mut add_header_convertor = AddHeaderConvertor::new("pub ");
        let name = "Test";
        add_header_convertor.add(name);
        let dummy_alias_type = AliasTypeStructure::new(name, make_string_type());
        add_header_convertor.convert(&mut acc, &dummy_alias_type);
        assert_eq!(acc, tobe)
    }
    #[test]
    fn test_add_header_case_not_containe() {
        let mut acc = String::from("type Test = String;");
        let tobe = acc.clone();
        let mut add_header_convertor = AddHeaderConvertor::new("pub ");
        let name = "Test";
        add_header_convertor.add(name);
        let dummy_alias_type = AliasTypeStructure::new("NotContaine", make_string_type());
        add_header_convertor.convert(&mut acc, &dummy_alias_type);
        assert_eq!(acc, tobe)
    }
    #[test]
    fn test_black_list_convertor_case_containe() {
        let name = "Test";
        let mut acc = String::from("type Test = String;");
        let tobe = String::new();
        let mut black_list = BlackListConvertor::new();
        black_list.add(name);
        let dummy_alias_type = AliasTypeStructure::new(name, make_string_type());
        black_list.convert(&mut acc, &dummy_alias_type);
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_black_list_convertor_case_not_containe() {
        let name = "Test";
        let mut acc = String::from("type Test = String;");
        let tobe = acc.clone();
        let black_list = BlackListConvertor::new();
        let dummy_alias_type = AliasTypeStructure::new(name, make_string_type());
        black_list.convert(&mut acc, &dummy_alias_type);
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_white_list_convertor_case_containe() {
        let name = "Test";
        let mut acc = String::from("type Test = String;");
        let tobe = acc.clone();
        let mut white_list = WhiteListConvertor::new();
        white_list.add(name);
        let dummy_alias_type = AliasTypeStructure::new(name, make_string_type());
        white_list.convert(&mut acc, &dummy_alias_type);
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_white_list_convertor_case_not_containe() {
        let name = "Test";
        let mut acc = String::from("type Test = String;");
        let tobe = String::new();
        let white_list = WhiteListConvertor::new();
        let dummy_alias_type = AliasTypeStructure::new(name, make_string_type());
        white_list.convert(&mut acc, &dummy_alias_type);
        assert_eq!(acc, tobe);
    }
}
