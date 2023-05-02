use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub trait ToDeclarePartConvertor: Sized {
    fn clone(&self) -> Self;
    fn to_declare_part(&self) -> Box<Self> {
        Box::new(self.clone())
    }
}
struct ConvertorMapStore {
    store: Rc<RefCell<HashMap<String, String>>>,
    all: Rc<RefCell<Vec<String>>>,
}
impl ConvertorMapStore {
    pub fn new() -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new())),
            all: Rc::new(RefCell::new(Vec::new())),
        }
    }
    pub fn add(&mut self, match_type_name: impl Into<String>, value: impl Into<String>) {
        self.store
            .borrow_mut()
            .insert(match_type_name.into(), value.into());
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
pub struct AddCommentConvertor {
    comment_identify: String,
    store: ConvertorMapStore,
}
impl AddCommentConvertor {
    pub fn new(comment_identify: impl Into<String>) -> Self {
        Self {
            comment_identify: comment_identify.into(),
            store: ConvertorMapStore::new(),
        }
    }
    pub fn add(&mut self, match_type_name: impl Into<String>, value: impl Into<String>) {
        self.store.add(match_type_name, value.into());
    }
    pub fn add_all(&mut self, value: impl Into<String>) {
        self.store.add_all(value.into());
    }
}
impl ToDeclarePartConvertor for AddCommentConvertor {
    fn clone(&self) -> Self {
        Self {
            comment_identify: self.comment_identify.clone(),
            store: self.store.clone(),
        }
    }
}
struct ConvertorStore {
    is_all: bool,
    store: Rc<RefCell<Vec<String>>>,
}
impl ConvertorStore {
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
    fn add(&mut self, type_name: impl Into<String>) {
        self.store.borrow_mut().push(type_name.into())
    }
    fn contain_list(&self, type_name: &str) -> bool {
        self.is_all || self.store.borrow().iter().any(|name| name == type_name)
    }
}
pub struct AddHeaderConvertor {
    header: String,
    store: ConvertorStore,
}
impl AddHeaderConvertor {
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            store: ConvertorStore::new(),
        }
    }
    pub fn add(&mut self, type_name: impl Into<String>) {
        self.store.add(type_name.into());
    }
    pub fn all(&mut self) {
        self.store.all();
    }
}
impl ToDeclarePartConvertor for AddHeaderConvertor {
    fn clone(&self) -> Self {
        Self {
            header: self.header.clone(),
            store: self.store.clone(),
        }
    }
}

pub struct BlackListConvertor {
    store: ConvertorStore,
}

impl BlackListConvertor {
    pub fn new() -> Self {
        Self {
            store: ConvertorStore::new(),
        }
    }
    pub fn add(&mut self, type_name: impl Into<String>) {
        self.store.add(type_name);
    }
}
impl ToDeclarePartConvertor for BlackListConvertor {
    fn clone(&self) -> Self {
        BlackListConvertor {
            store: self.store.clone(),
        }
    }
}
impl ToDeclarePartConvertor for WhiteListConvertor {
    fn clone(&self) -> Self {
        WhiteListConvertor {
            store: self.store.clone(),
        }
    }
}

pub struct WhiteListConvertor {
    store: ConvertorStore,
}

impl WhiteListConvertor {
    pub fn new() -> Self {
        Self {
            store: ConvertorStore::new(),
        }
    }
    pub fn add(&mut self, type_name: impl Into<String>) {
        self.store.add(type_name)
    }
}

pub mod composite_type {

    use super::*;
    use crate::customizable::declare_part_generator::{
        CompositeTypeDeclareConvertor, TypeIdentifyConvertor,
    };
    impl TypeIdentifyConvertor for AddCommentConvertor {
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
    impl TypeIdentifyConvertor for AddHeaderConvertor {
        fn convert(
            &self,
            acc: &mut String,
            type_name: &structure::parts::type_name::TypeName,
        ) -> () {
            if self.store.contain_list(type_name.as_str()) {
                *acc = format!("{}{}", self.header, acc)
            }
        }
    }

    impl CompositeTypeDeclareConvertor for BlackListConvertor {
        fn convert(
            &self,
            acc: Option<String>,
            composite_type: &structure::composite_type_structure::CompositeTypeStructure,
        ) -> Option<String> {
            if self.store.contain_list(composite_type.type_name().as_str()) {
                None
            } else {
                acc
            }
        }
    }

    impl CompositeTypeDeclareConvertor for AddHeaderConvertor {
        fn convert(
            &self,
            acc: Option<String>,
            composite_type: &structure::composite_type_structure::CompositeTypeStructure,
        ) -> Option<String> {
            if self.store.contain_list(composite_type.type_name().as_str()) {
                if let Some(acc) = acc {
                    return Some(format!("{}\n{}", self.header, acc));
                }
            }
            acc
        }
    }
    impl CompositeTypeDeclareConvertor for WhiteListConvertor {
        fn convert(
            &self,
            acc: Option<String>,
            composite_type: &structure::composite_type_structure::CompositeTypeStructure,
        ) -> Option<String> {
            if !self.store.contain_list(composite_type.type_name().as_str()) {
                None
            } else {
                acc
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

    impl AliasTypeDeclareConvertor for AddHeaderConvertor {
        fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
            if self.store.contain_list(alias_type.type_name().as_str()) {
                *acc = format!("{}\n{}", self.header, acc)
            }
        }
    }
    impl AliasTypeIdentifyConvertor for AddHeaderConvertor {
        fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
            if self.store.contain_list(alias_type.type_name().as_str()) {
                *acc = format!("{}{}", self.header, acc)
            }
        }
    }
    impl AliasTypeDeclareConvertor for BlackListConvertor {
        fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
            if self.store.contain_list(alias_type.type_name().as_str()) {
                *acc = String::new()
            }
        }
    }

    impl AliasTypeDeclareConvertor for WhiteListConvertor {
        fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
            if !self.store.contain_list(alias_type.type_name().as_str()) {
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
mod composite_convetor_tests {
    #[allow(non_snake_case)]
    mod case_where_the_name_of_type_is_Test_and_the_property_has_an_id_property_of_type_usize {
        fn factory() -> String {
            String::from("struct Test {id:usize}")
        }
        use structure::parts::type_name::TypeName;

        use crate::customizable::declare_part_convetors::AddCommentConvertor;
        #[test]
        #[allow(non_snake_case)]
        fn using_AddCommentConvertor_add_all_will_add_comments_to_all_type_definition_descriptions()
        {
            use crate::customizable::declare_part_generator::TypeIdentifyConvertor;
            let mut description = factory();
            let comment = "this comment!";
            let mut sut = AddCommentConvertor::new("// ");
            sut.add_all(comment);
            let tobe = format!("// {}\n{}", comment, description);

            sut.convert(&mut description, &TypeName::from("Test"));

            assert_eq!(tobe, description);
        }
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
    fn test_add_comment_convertor_case_contain() {
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
    fn test_add_header_convertor_case_contain() {
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
    fn test_add_header_convertor_case_not_contain() {
        use crate::customizable::declare_part_generator::TypeIdentifyConvertor;
        let name = "Test";
        let mut acc = String::from("struct Test {id:usize}");
        let tobe = acc.clone();
        let mut add_header = AddHeaderConvertor::new("pub ");
        add_header.add(name);
        add_header.convert(&mut acc, &TypeName::from("Notcontain"));
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_black_list_convertor_case_contain() {
        use crate::customizable::declare_part_generator::CompositeTypeDeclareConvertor;
        let name = "Test";
        let acc = String::from("struct Test {id:usize}");
        let mut black_list = BlackListConvertor::new();
        black_list.add(name);
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        let result = black_list.convert(Some(acc), &dummy_composite_type);
        assert_eq!(result, None);
    }
    #[test]
    fn test_black_list_convertor_case_not_contain() {
        use crate::customizable::declare_part_generator::CompositeTypeDeclareConvertor;
        let name = "Test";
        let acc = String::from("struct Test {id:usize}");
        let tobe = acc.clone();
        let black_list = BlackListConvertor::new();
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        let result = black_list.convert(Some(acc), &dummy_composite_type);
        assert_eq!(result.unwrap(), tobe);
    }
    #[test]
    fn test_white_list_convertor_case_contain() {
        use crate::customizable::declare_part_generator::CompositeTypeDeclareConvertor;
        let name = "Test";
        let acc = String::from("struct Test {id:usize}");
        let tobe = acc.clone();
        let mut white_list = WhiteListConvertor::new();
        white_list.add(name);
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        let result = white_list.convert(Some(acc), &dummy_composite_type);
        assert_eq!(result.unwrap(), tobe);
    }
    #[test]
    fn test_white_list_convertor_case_not_contain() {
        use crate::customizable::declare_part_generator::CompositeTypeDeclareConvertor;
        let name = "Test";
        let acc = String::from("struct Test {id:usize}");
        let white_list = WhiteListConvertor::new();
        let dummy_composite_type = CompositeTypeStructure::new(name, BTreeMap::new());
        let result = white_list.convert(Some(acc), &dummy_composite_type);
        assert_eq!(result, None);
    }
}

#[cfg(test)]
mod alias_case_test {

    use super::*;
    use structure::{
        alias_type_structure::AliasTypeStructure,
        parts::property_type::property_type_factories::make_string_type,
    };
    #[test]
    fn test_add_header_case_contain() {
        use crate::customizable::declare_part_generator::AliasTypeIdentifyConvertor;
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
    fn test_add_header_case_not_contain() {
        use crate::customizable::declare_part_generator::AliasTypeIdentifyConvertor;
        let mut acc = String::from("type Test = String;");
        let tobe = acc.clone();
        let mut add_header_convertor = AddHeaderConvertor::new("pub ");
        let name = "Test";
        add_header_convertor.add(name);
        let dummy_alias_type = AliasTypeStructure::new("Notcontain", make_string_type());
        add_header_convertor.convert(&mut acc, &dummy_alias_type);
        assert_eq!(acc, tobe)
    }
    #[test]
    fn test_black_list_convertor_case_contain() {
        use crate::customizable::declare_part_generator::AliasTypeDeclareConvertor;
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
    fn test_black_list_convertor_case_not_contain() {
        use crate::customizable::declare_part_generator::AliasTypeDeclareConvertor;
        let name = "Test";
        let mut acc = String::from("type Test = String;");
        let tobe = acc.clone();
        let black_list = BlackListConvertor::new();
        let dummy_alias_type = AliasTypeStructure::new(name, make_string_type());
        black_list.convert(&mut acc, &dummy_alias_type);
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_white_list_convertor_case_contain() {
        use crate::customizable::declare_part_generator::AliasTypeDeclareConvertor;
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
    fn test_white_list_convertor_case_not_contain() {
        use crate::customizable::declare_part_generator::AliasTypeDeclareConvertor;
        let name = "Test";
        let mut acc = String::from("type Test = String;");
        let tobe = String::new();
        let white_list = WhiteListConvertor::new();
        let dummy_alias_type = AliasTypeStructure::new(name, make_string_type());
        white_list.convert(&mut acc, &dummy_alias_type);
        assert_eq!(acc, tobe);
    }
}
