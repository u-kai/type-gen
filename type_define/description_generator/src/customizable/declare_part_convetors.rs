use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub trait ToDeclarePartConvertor: Sized {
    fn clone(&self) -> Self;
    fn to_declare_part(&self) -> Box<Self> {
        Box::new(self.clone())
    }
}
struct ConvertorMapStore<'a> {
    store: Rc<RefCell<HashMap<&'a str, String>>>,
    all: Rc<RefCell<Vec<String>>>,
}
impl<'a> ConvertorMapStore<'a> {
    pub fn new() -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new())),
            all: Rc::new(RefCell::new(Vec::new())),
        }
    }
    pub fn add(&mut self, match_type_name: &'a str, value: impl Into<String>) {
        self.store
            .borrow_mut()
            .insert(match_type_name, value.into());
    }
    pub fn add_all(&mut self, value: impl Into<String>) {
        self.all.borrow_mut().push(value.into());
    }
    pub fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            all: self.all.clone(),
        }
    }
}
pub struct AddCommentConvertor<'a> {
    comment_identify: &'a str,
    store: ConvertorMapStore<'a>,
}
impl<'a> AddCommentConvertor<'a> {
    pub fn new(comment_identify: &'a str) -> Self {
        Self {
            comment_identify,
            store: ConvertorMapStore::new(),
        }
    }
    pub fn add(&mut self, match_type_name: &'a str, value: impl Into<String>) {
        self.store.add(match_type_name, value.into());
    }
    pub fn add_all(&mut self, value: impl Into<String>) {
        self.store.add_all(value.into());
    }
}
impl<'a> ToDeclarePartConvertor for AddCommentConvertor<'a> {
    fn clone(&self) -> Self {
        Self {
            comment_identify: self.comment_identify,
            store: self.store.clone(),
        }
    }
}
struct ConvertorStore<'a> {
    is_all: bool,
    store: Rc<RefCell<Vec<&'a str>>>,
}
impl<'a> ConvertorStore<'a> {
    fn new() -> Self {
        Self {
            is_all: false,
            store: Rc::new(RefCell::new(Vec::new())),
        }
    }
    fn clone(&self) -> Self {
        ConvertorStore {
            is_all: self.is_all,
            store: self.store.clone(),
        }
    }
    fn all(&mut self) {
        self.is_all = true
    }
    fn add(&mut self, type_name: &'a str) {
        self.store.borrow_mut().push(type_name)
    }
    fn containe_list(&self, type_name: &str) -> bool {
        self.is_all || self.store.borrow().contains(&type_name)
    }
}
pub struct AddHeaderConvertor<'a> {
    header: String,
    store: ConvertorStore<'a>,
}
impl<'a> AddHeaderConvertor<'a> {
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            store: ConvertorStore::new(),
        }
    }
    pub fn add(&mut self, type_name: impl Into<&'a str>) {
        self.store.add(type_name.into());
    }
    pub fn all(&mut self) {
        self.store.all();
    }
}
impl<'a> ToDeclarePartConvertor for AddHeaderConvertor<'a> {
    fn clone(&self) -> Self {
        Self {
            header: self.header.clone(),
            store: self.store.clone(),
        }
    }
}

pub struct BlackListConvertor<'a> {
    store: ConvertorStore<'a>,
}

impl<'a> BlackListConvertor<'a> {
    pub fn new() -> Self {
        Self {
            store: ConvertorStore::new(),
        }
    }
    pub fn add(&mut self, type_name: &'a str) {
        self.store.add(type_name);
    }
}
impl<'a> ToDeclarePartConvertor for BlackListConvertor<'a> {
    fn clone(&self) -> Self {
        BlackListConvertor {
            store: self.store.clone(),
        }
    }
}
impl<'a> ToDeclarePartConvertor for WhiteListConvertor<'a> {
    fn clone(&self) -> Self {
        WhiteListConvertor {
            store: self.store.clone(),
        }
    }
}

pub struct WhiteListConvertor<'a> {
    store: ConvertorStore<'a>,
}

impl<'a> WhiteListConvertor<'a> {
    pub fn new() -> Self {
        Self {
            store: ConvertorStore::new(),
        }
    }
    pub fn add(&mut self, type_name: &'a str) {
        self.store.add(type_name)
    }
}

pub mod composite_type {

    use super::*;
    use crate::customizable::declare_part_generator::{
        CompositeTypeDeclareConvertor, TypeIdentifyConvertor,
    };
    impl<'a> TypeIdentifyConvertor for AddCommentConvertor<'a> {
        fn convert(
            &self,
            acc: &mut String,
            type_name: &structure::parts::type_name::TypeName,
        ) -> () {
            self.store.all.borrow().iter().for_each(|comment| {
                *acc = format!("{}{}\n{}", self.comment_identify, comment, acc)
            });
            if let Some(value) = self.store.store.borrow().get(type_name.as_str()) {
                *acc = format!("{}{}\n{}", self.comment_identify, value, acc);
            }
        }
    }
    impl<'a> TypeIdentifyConvertor for AddHeaderConvertor<'a> {
        fn convert(
            &self,
            acc: &mut String,
            type_name: &structure::parts::type_name::TypeName,
        ) -> () {
            if self.store.containe_list(type_name.as_str()) {
                *acc = format!("{}{}", self.header, acc)
            }
        }
    }

    impl<'a> CompositeTypeDeclareConvertor for BlackListConvertor<'a> {
        fn convert(
            &self,
            acc: &mut String,
            composite_type: &structure::composite_type_structure::CompositeTypeStructure,
        ) -> () {
            if self
                .store
                .containe_list(composite_type.type_name().as_str())
            {
                *acc = String::new()
            }
        }
    }

    impl<'a> CompositeTypeDeclareConvertor for AddHeaderConvertor<'a> {
        fn convert(
            &self,
            acc: &mut String,
            composite_type: &structure::composite_type_structure::CompositeTypeStructure,
        ) -> () {
            if self
                .store
                .containe_list(composite_type.type_name().as_str())
            {
                *acc = format!("{}\n{}", self.header, acc)
            }
        }
    }
    impl<'a> CompositeTypeDeclareConvertor for WhiteListConvertor<'a> {
        fn convert(
            &self,
            acc: &mut String,
            composite_type: &structure::composite_type_structure::CompositeTypeStructure,
        ) -> () {
            if !self
                .store
                .containe_list(composite_type.type_name().as_str())
            {
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
            if self.store.containe_list(alias_type.type_name().as_str()) {
                *acc = format!("{}{}", self.header, acc)
            }
        }
    }
    impl<'a> AliasTypeDeclareConvertor for BlackListConvertor<'a> {
        fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
            if self.store.containe_list(alias_type.type_name().as_str()) {
                *acc = String::new()
            }
        }
    }
    impl<'a> AliasTypeDeclareConvertor for AddHeaderConvertor<'a> {
        fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
            if self.store.containe_list(alias_type.type_name().as_str()) {
                *acc = format!("{}{}", self.header, acc)
            }
        }
    }

    impl<'a> AliasTypeDeclareConvertor for WhiteListConvertor<'a> {
        fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
            if !self.store.containe_list(alias_type.type_name().as_str()) {
                *acc = String::new()
            }
        }
    }
}
#[cfg(test)]
mod integration_test {
    use crate::{
        customizable::declare_part_generator::{
            CustomizableAliasTypeDeclareGenerator, CustomizableCompositeTypeDeclareGenerator,
        },
        type_mapper::fake_mapper::FakeTypeMapper,
    };

    use super::*;

    #[test]
    fn test_declare_part_generator() {
        let white_list = WhiteListConvertor::new();
        let black_list = BlackListConvertor::new();
        let add_header = AddHeaderConvertor::new("pub ");
        let mut composite_type =
            CustomizableCompositeTypeDeclareGenerator::new_curly_bracket_lang("class");
        composite_type.add_description_convertor(white_list.to_declare_part());
        composite_type.add_description_convertor(black_list.to_declare_part());
        composite_type.add_type_identify_convertor(add_header.to_declare_part());
        let mut alias_type: CustomizableAliasTypeDeclareGenerator<
            FakeTypeMapper,
            fn(&str, &structure::parts::type_name::TypeName, String) -> String,
        > = CustomizableAliasTypeDeclareGenerator::defalut("class");
        alias_type.add_description_convertor(white_list.to_declare_part());
        alias_type.add_description_convertor(black_list.to_declare_part());
        alias_type.add_type_identify_convertor(add_header.to_declare_part());
    }
}
#[cfg(test)]
mod composite_case_test {

    use super::*;
    use std::collections::BTreeMap;
    use structure::{composite_type_structure::CompositeTypeStructure, parts::type_name::TypeName};
    #[test]
    fn test_add_comment_convertor_case_all() {
        use crate::customizable::declare_part_generator::TypeIdentifyConvertor;
        let name = "Test";
        let mut acc = String::from("struct Test {id:usize}");
        let comment = "this comment!";
        let tobe = format!("// {}\n{}", comment, acc);
        let mut add_comment = AddCommentConvertor::new("// ");
        add_comment.add_all(comment);
        add_comment.convert(&mut acc, &TypeName::from(name));
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_add_comment_convertor_case_containe() {
        use crate::customizable::declare_part_generator::TypeIdentifyConvertor;
        let name = "Test";
        let mut acc = String::from("struct Test {id:usize}");
        let comment = "this comment!";
        let tobe = format!("// {}\n{}", comment, acc);
        let mut add_comment = AddCommentConvertor::new("// ");
        add_comment.add(name, comment);
        add_comment.convert(&mut acc, &TypeName::from(name));
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_add_header_convertor_case_all() {
        use crate::customizable::declare_part_generator::TypeIdentifyConvertor;
        let name = "Test";
        let mut acc = String::from("struct Test {id:usize}");
        let tobe = format!("pub {}", acc);
        let mut add_header = AddHeaderConvertor::new("pub ");
        add_header.all();
        add_header.convert(&mut acc, &TypeName::from(name));
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_add_header_convertor_case_containe() {
        use crate::customizable::declare_part_generator::TypeIdentifyConvertor;
        let name = "Test";
        let mut acc = String::from("struct Test {id:usize}");
        let tobe = format!("pub {}", acc);
        let mut add_header = AddHeaderConvertor::new("pub ");
        add_header.add(name);
        add_header.convert(&mut acc, &TypeName::from(name));
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_add_header_convertor_case_not_containe() {
        use crate::customizable::declare_part_generator::TypeIdentifyConvertor;
        let name = "Test";
        let mut acc = String::from("struct Test {id:usize}");
        let tobe = acc.clone();
        let mut add_header = AddHeaderConvertor::new("pub ");
        add_header.add(name);
        add_header.convert(&mut acc, &TypeName::from("NotContaine"));
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_black_list_convertor_case_containe() {
        use crate::customizable::declare_part_generator::CompositeTypeDeclareConvertor;
        let name = "Test";
        let mut acc = String::from("struct Test {id:usize}");
        let tobe = String::new();
        let mut black_list = BlackListConvertor::new();
        black_list.add(name);
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        black_list.convert(&mut acc, &dummy_composite_type);
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_black_list_convertor_case_not_containe() {
        use crate::customizable::declare_part_generator::CompositeTypeDeclareConvertor;
        let name = "Test";
        let mut acc = String::from("struct Test {id:usize}");
        let tobe = acc.clone();
        let black_list = BlackListConvertor::new();
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        black_list.convert(&mut acc, &dummy_composite_type);
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_white_list_convertor_case_containe() {
        use crate::customizable::declare_part_generator::CompositeTypeDeclareConvertor;
        let name = "Test";
        let mut acc = String::from("struct Test {id:usize}");
        let tobe = acc.clone();
        let mut white_list = WhiteListConvertor::new();
        white_list.add(name);
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        white_list.convert(&mut acc, &dummy_composite_type);
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_white_list_convertor_case_not_containe() {
        use crate::customizable::declare_part_generator::CompositeTypeDeclareConvertor;
        let name = "Test";
        let mut acc = String::from("struct Test {id:usize}");
        let tobe = String::new();
        let white_list = WhiteListConvertor::new();
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        white_list.convert(&mut acc, &dummy_composite_type);
        assert_eq!(acc, tobe);
    }
}

#[cfg(test)]
mod alias_case_test {
    use crate::customizable::declare_part_generator::AliasTypeDeclareConvertor;

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
