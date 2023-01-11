use super::declare_part_generator::CompositeTypeDeclareConvertor;

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
#[cfg(test)]
mod composite_case_test {
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
