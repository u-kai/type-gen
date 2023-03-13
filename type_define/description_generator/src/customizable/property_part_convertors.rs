use npc::fns::{to_camel, to_constant, to_pascal, to_snake};
use structure::parts::{property_key::PropertyKey, type_name::TypeName};

use crate::type_mapper::TypeMapper;

use super::property_part_generator::Convertor;

struct PropertyPartMatchConditionStore {
    is_all: bool,
    match_type_names: Vec<String>,
    match_propety_keys: Vec<String>,
    match_type_name_and_propety_keys: Vec<(String, String)>,
}
impl PropertyPartMatchConditionStore {
    fn new() -> Self {
        Self {
            is_all: false,
            match_propety_keys: Vec::new(),
            match_type_names: Vec::new(),
            match_type_name_and_propety_keys: Vec::new(),
        }
    }
    fn set_all(&mut self) {
        self.is_all = true
    }
    fn add_match_type_name(&mut self, type_name: impl Into<String>) {
        self.match_type_names.push(type_name.into());
    }
    fn add_match_property_key(&mut self, property_key: impl Into<String>) {
        self.match_propety_keys.push(property_key.into());
    }
    fn add_match_type_name_and_property_key(
        &mut self,
        type_name: impl Into<String>,
        property_key: impl Into<String>,
    ) {
        self.match_type_name_and_propety_keys
            .push((type_name.into(), property_key.into()));
    }
    fn is_match(&self, type_name: &str, property_key: &str) -> bool {
        self.is_all
            || self.match_type_names.iter().any(|s| s == type_name)
            || self.match_propety_keys.iter().any(|s| s == &property_key)
            || self
                .match_type_name_and_propety_keys
                .iter()
                .any(|s| (s.0.as_str(), s.1.as_str()) == (type_name, property_key))
    }
}
macro_rules! impl_match_condition_store_methods {
    ($($t:ident),*) => {
        $(impl $t {
            pub fn set_all(&mut self) {
                self.store.set_all()
            }
            pub fn add_match_type_name(&mut self, type_name: impl Into<String>) {
                self.store.add_match_type_name(type_name);
            }
            pub fn add_match_property_key(&mut self, property_key: impl Into<String>) {
                self.store.add_match_property_key(property_key);
            }
            pub fn add_match_type_name_and_property_key(&mut self, type_name:impl Into<String>, property_key: impl Into<String>) {
                self.store.add_match_type_name_and_property_key(type_name, property_key);
            }
        })*
    };
    ($($t:ident),*,) => {
        impl_match_condition_store_methods!($($t),*);
    };
}

pub struct RenameConvertor {
    principal: Principal,
    store: PropertyPartMatchConditionStore,
}
impl RenameConvertor {
    pub fn new(principal: Principal) -> Self {
        Self {
            principal,
            store: PropertyPartMatchConditionStore::new(),
        }
    }
}
impl<M> Convertor<M> for RenameConvertor
where
    M: TypeMapper,
{
    fn convert(
        &self,
        acc: &mut String,
        type_name: &TypeName,
        property_key: &PropertyKey,
        _: &structure::parts::property_type::PropertyType,
        _: &M,
    ) -> () {
        if self
            .store
            .is_match(type_name.as_str(), property_key.as_str())
        {
            *acc = match self.principal {
                Principal::Camel => to_camel(acc),
                Principal::Snake => to_snake(acc),
                Principal::Constant => to_constant(acc),
                Principal::Pascal => to_pascal(acc),
            };
        }
    }
}
pub enum Principal {
    Snake,
    Camel,
    Pascal,
    Constant,
}
pub struct ToOptionalConvertor {
    store: PropertyPartMatchConditionStore,
}
impl ToOptionalConvertor {
    pub fn new() -> Self {
        Self {
            store: PropertyPartMatchConditionStore::new(),
        }
    }
}
pub struct AddHeaderConvertor {
    header: String,
    store: PropertyPartMatchConditionStore,
}
impl AddHeaderConvertor {
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            store: PropertyPartMatchConditionStore::new(),
        }
    }
}
pub struct AddLastSideConvertor {
    added: String,
    store: PropertyPartMatchConditionStore,
}
impl AddLastSideConvertor {
    pub fn new(added: impl Into<String>) -> Self {
        Self {
            added: added.into(),
            store: PropertyPartMatchConditionStore::new(),
        }
    }
}
pub struct AddLeftSideConvertor {
    added: String,
    store: PropertyPartMatchConditionStore,
}
impl AddLeftSideConvertor {
    pub fn new(added: impl Into<String>) -> Self {
        Self {
            added: added.into(),
            store: PropertyPartMatchConditionStore::new(),
        }
    }
}
pub struct AddRightSideConvertor {
    added: String,
    store: PropertyPartMatchConditionStore,
}
impl AddRightSideConvertor {
    pub fn new(added: impl Into<String>) -> Self {
        Self {
            added: added.into(),
            store: PropertyPartMatchConditionStore::new(),
        }
    }
}
pub struct BlackListConvertor {
    store: PropertyPartMatchConditionStore,
}
impl BlackListConvertor {
    pub fn new() -> Self {
        Self {
            store: PropertyPartMatchConditionStore::new(),
        }
    }
}
pub struct WhiteListConvertor {
    store: PropertyPartMatchConditionStore,
}
impl WhiteListConvertor {
    pub fn new() -> Self {
        Self {
            store: PropertyPartMatchConditionStore::new(),
        }
    }
}
pub struct CannotUseCharConvertor {
    removes: Vec<char>,
    cannot_uses: Vec<char>,
}
impl CannotUseCharConvertor {
    pub fn new() -> Self {
        Self {
            removes: Vec::new(),
            cannot_uses: vec![
                ':', ';', '#', '$', '%', '&', '~', '=', ',', '\"', '\'', '{', '}', '?', '!', '<',
                '>', '[', ']', '*', '^', '-',
            ],
        }
    }
    fn replace_cannot_use_char(&self, str: &str) -> String {
        str.chars().fold(String::new(), |mut acc, c| {
            if self.containe_cannot_use_char(c) {
                acc
            } else {
                acc.push(c);
                acc
            }
        })
    }
    fn containe_cannot_use_char(&self, c: char) -> bool {
        self.cannot_uses.contains(&c) && !self.removes.contains(&c)
    }
    pub fn add(&mut self, c: char) {
        self.cannot_uses.push(c);
    }
    pub fn remove(&mut self, c: char) {
        self.removes.push(c);
    }
}
impl_match_condition_store_methods!(
    AddHeaderConvertor,
    AddLeftSideConvertor,
    ToOptionalConvertor,
    AddRightSideConvertor,
    AddLastSideConvertor,
    RenameConvertor,
    BlackListConvertor,
    WhiteListConvertor
);
pub mod description_convertors {

    use crate::customizable::property_part_generator::DescriptionConvertor;

    use super::*;
    impl<M> DescriptionConvertor<M> for WhiteListConvertor
    where
        M: TypeMapper,
    {
        fn convert(
            &self,
            acc: Option<String>,
            type_name: &TypeName,
            property_key: &PropertyKey,
            _: &structure::parts::property_type::PropertyType,
            _: &M,
        ) -> Option<String> {
            if self
                .store
                .is_match(type_name.as_str(), property_key.as_str())
            {
                acc
            } else {
                None
            }
        }
    }
    impl<M> DescriptionConvertor<M> for BlackListConvertor
    where
        M: TypeMapper,
    {
        fn convert(
            &self,
            acc: Option<String>,
            type_name: &TypeName,
            property_key: &PropertyKey,
            _: &structure::parts::property_type::PropertyType,
            _: &M,
        ) -> Option<String> {
            if self
                .store
                .is_match(type_name.as_str(), property_key.as_str())
            {
                None
            } else {
                acc
            }
        }
    }
    impl<M> DescriptionConvertor<M> for AddHeaderConvertor
    where
        M: TypeMapper,
    {
        fn convert(
            &self,
            acc: Option<String>,
            type_name: &TypeName,
            property_key: &PropertyKey,
            _: &structure::parts::property_type::PropertyType,
            _: &M,
        ) -> Option<String> {
            if self
                .store
                .is_match(type_name.as_str(), property_key.as_str())
            {
                if let Some(acc) = acc {
                    return Some(format!("{}\n{}", self.header, acc));
                }
            }
            acc
        }
    }
    impl<M> DescriptionConvertor<M> for AddLeftSideConvertor
    where
        M: TypeMapper,
    {
        fn convert(
            &self,
            acc: Option<String>,
            type_name: &TypeName,
            property_key: &PropertyKey,
            _: &structure::parts::property_type::PropertyType,
            _: &M,
        ) -> Option<String> {
            if self
                .store
                .is_match(type_name.as_str(), property_key.as_str())
            {
                if let Some(acc) = acc {
                    let mut result = acc.split("\n").fold(String::new(), |acc, line| {
                        format!("{}{}{}\n", acc, self.added, line)
                    });
                    result.remove(result.len() - 1);
                    return Some(result);
                }
            }
            acc
        }
    }
    impl<M> DescriptionConvertor<M> for ToOptionalConvertor
    where
        M: TypeMapper,
    {
        fn convert(
            &self,
            acc: Option<String>,
            type_name: &TypeName,
            property_key: &PropertyKey,
            _: &structure::parts::property_type::PropertyType,
            mapper: &M,
        ) -> Option<String> {
            if self
                .store
                .is_match(type_name.as_str(), property_key.as_str())
            {
                if let Some(acc) = acc {
                    return Some(mapper.case_optional_type(acc.clone()));
                }
            }
            acc
        }
    }
    impl<M> DescriptionConvertor<M> for AddRightSideConvertor
    where
        M: TypeMapper,
    {
        fn convert(
            &self,
            acc: Option<String>,
            type_name: &TypeName,
            property_key: &PropertyKey,
            _: &structure::parts::property_type::PropertyType,
            _: &M,
        ) -> Option<String> {
            if self
                .store
                .is_match(type_name.as_str(), property_key.as_str())
            {
                if let Some(mut acc) = acc {
                    acc.split("\n").fold(String::new(), |acc, line| {
                        format!("{}{}{}\n", acc, line, self.added)
                    });
                    acc.remove(acc.len() - 1);
                    return Some(acc);
                }
            }
            acc
        }
    }
    impl<M> DescriptionConvertor<M> for AddLastSideConvertor
    where
        M: TypeMapper,
    {
        fn convert(
            &self,
            acc: Option<String>,
            type_name: &TypeName,
            property_key: &PropertyKey,
            _: &structure::parts::property_type::PropertyType,
            _: &M,
        ) -> Option<String> {
            if self
                .store
                .is_match(type_name.as_str(), property_key.as_str())
            {
                if let Some(acc) = acc {
                    return Some(format!("{}{}", acc, self.added));
                }
            }
            acc
        }
    }
    impl<M> DescriptionConvertor<M> for RenameConvertor
    where
        M: TypeMapper,
    {
        fn convert(
            &self,
            acc: Option<String>,
            type_name: &TypeName,
            property_key: &PropertyKey,
            _: &structure::parts::property_type::PropertyType,
            _: &M,
        ) -> Option<String> {
            if self
                .store
                .is_match(type_name.as_str(), property_key.as_str())
            {
                if let Some(acc) = acc {
                    return Some(match self.principal {
                        Principal::Camel => to_camel(&acc),
                        Principal::Snake => to_snake(&acc),
                        Principal::Constant => to_constant(&acc),
                        Principal::Pascal => to_pascal(&acc),
                    });
                }
            }
            acc
        }
    }
}
impl<M> Convertor<M> for AddLastSideConvertor
where
    M: TypeMapper,
{
    fn convert(
        &self,
        acc: &mut String,
        type_name: &TypeName,
        property_key: &PropertyKey,
        _: &structure::parts::property_type::PropertyType,
        _: &M,
    ) -> () {
        if self
            .store
            .is_match(type_name.as_str(), property_key.as_str())
        {
            *acc = format!("{}{}", acc, self.added);
        }
    }
}
impl<M> Convertor<M> for CannotUseCharConvertor
where
    M: TypeMapper,
{
    fn convert(
        &self,
        acc: &mut String,
        _: &TypeName,
        _: &PropertyKey,
        _: &structure::parts::property_type::PropertyType,
        _: &M,
    ) -> () {
        *acc = self.replace_cannot_use_char(&acc);
    }
}
impl<M> Convertor<M> for ToOptionalConvertor
where
    M: TypeMapper,
{
    fn convert(
        &self,
        acc: &mut String,
        type_name: &TypeName,
        property_key: &PropertyKey,
        _: &structure::parts::property_type::PropertyType,
        mapper: &M,
    ) -> () {
        if self
            .store
            .is_match(type_name.as_str(), property_key.as_str())
        {
            *acc = mapper.case_optional_type(acc.clone());
        }
    }
}
impl<M> Convertor<M> for BlackListConvertor
where
    M: TypeMapper,
{
    fn convert(
        &self,
        acc: &mut String,
        type_name: &TypeName,
        property_key: &PropertyKey,
        _: &structure::parts::property_type::PropertyType,
        _: &M,
    ) -> () {
        if self
            .store
            .is_match(type_name.as_str(), property_key.as_str())
        {
            *acc = String::new()
        }
    }
}
impl<M> Convertor<M> for AddLeftSideConvertor
where
    M: TypeMapper,
{
    fn convert(
        &self,
        acc: &mut String,
        type_name: &TypeName,
        property_key: &PropertyKey,
        _: &structure::parts::property_type::PropertyType,
        _: &M,
    ) -> () {
        if self
            .store
            .is_match(type_name.as_str(), property_key.as_str())
        {
            *acc = acc.split("\n").fold(String::new(), |acc, line| {
                format!("{}{}{}\n", acc, self.added, line)
            });
            acc.remove(acc.len() - 1);
            // * acc = format!("{}{}", self.added, acc.split("\n").)
        }
    }
}
impl<M> Convertor<M> for AddRightSideConvertor
where
    M: TypeMapper,
{
    fn convert(
        &self,
        acc: &mut String,
        type_name: &TypeName,
        property_key: &PropertyKey,
        _: &structure::parts::property_type::PropertyType,
        _: &M,
    ) -> () {
        if self
            .store
            .is_match(type_name.as_str(), property_key.as_str())
        {
            *acc = acc.split("\n").fold(String::new(), |acc, line| {
                format!("{}{}{}\n", acc, line, self.added)
            });
            acc.remove(acc.len() - 1);
        }
    }
}
impl<M> Convertor<M> for AddHeaderConvertor
where
    M: TypeMapper,
{
    fn convert(
        &self,
        acc: &mut String,
        type_name: &TypeName,
        property_key: &PropertyKey,
        _: &structure::parts::property_type::PropertyType,
        _: &M,
    ) -> () {
        if self
            .store
            .is_match(type_name.as_str(), property_key.as_str())
        {
            *acc = format!("{}\n{}", self.header, acc)
        }
    }
}
#[cfg(test)]
mod test {
    use crate::{
        customizable::property_part_convertors::{
            AddHeaderConvertor, AddLeftSideConvertor, AddRightSideConvertor, BlackListConvertor,
            CannotUseCharConvertor, Principal, RenameConvertor, ToOptionalConvertor,
        },
        type_mapper::fake_mapper::FakeTypeMapper,
    };
    use structure::parts::{
        property_key::PropertyKey, property_type::property_type_factories::make_usize_type,
        type_name::TypeName,
    };

    #[test]
    fn test_black_list_convertor() {
        use crate::customizable::property_part_generator::DescriptionConvertor;
        let acc = String::from("id:usize");
        let mut black_list = BlackListConvertor::new();
        black_list.add_match_property_key("id");
        let dummy_type_name = TypeName::from("");
        let type_key = PropertyKey::from("id");
        let dummy_property_type = make_usize_type();
        let result = black_list.convert(
            Some(acc),
            &dummy_type_name,
            &type_key,
            &dummy_property_type,
            &FakeTypeMapper,
        );
        assert_eq!(result, None);
    }
    #[test]
    fn test_replace_cannot_use_char_convertor() {
        use crate::customizable::property_part_generator::Convertor;
        let mut acc = String::from("id:value");
        let tobe = String::from("idvalue");
        let rename_convertor = CannotUseCharConvertor::new();
        let dummy_type_name = TypeName::from("");
        let type_key = PropertyKey::from("id:value");
        let dummy_property_type = make_usize_type();
        rename_convertor.convert(
            &mut acc,
            &dummy_type_name,
            &type_key,
            &dummy_property_type,
            &FakeTypeMapper,
        );
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_rename_convertor() {
        use crate::customizable::property_part_generator::Convertor;
        let mut acc = String::from("idValue");
        let tobe = String::from("id_value");
        let mut rename_convertor = RenameConvertor::new(Principal::Snake);
        rename_convertor.set_all();
        let dummy_type_name = TypeName::from("");
        let type_key = PropertyKey::from("idValue");
        let dummy_property_type = make_usize_type();
        rename_convertor.convert(
            &mut acc,
            &dummy_type_name,
            &type_key,
            &dummy_property_type,
            &FakeTypeMapper,
        );
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_to_optional_case_all() {
        use crate::customizable::property_part_generator::Convertor;
        let mut acc = String::from("usize");
        let tobe = String::from("Option<usize>");
        let mut to_optional_convertor = ToOptionalConvertor::new();
        to_optional_convertor.set_all();
        let dummy_type_name = TypeName::from("");
        let type_key = PropertyKey::from("id");
        let dummy_property_type = make_usize_type();
        to_optional_convertor.convert(
            &mut acc,
            &dummy_type_name,
            &type_key,
            &dummy_property_type,
            &FakeTypeMapper,
        );
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_add_header_case_all() {
        use crate::customizable::property_part_generator::Convertor;
        let space = "// this comment";
        let mut acc = String::from("id:usize");
        let tobe = format!("{}\n{}", space, acc);
        let mut add_header_convertor = AddHeaderConvertor::new(space);
        add_header_convertor.set_all();
        let dummy_type_name = TypeName::from("");
        let type_key = PropertyKey::from("id");
        let dummy_property_type = make_usize_type();
        add_header_convertor.convert(
            &mut acc,
            &dummy_type_name,
            &type_key,
            &dummy_property_type,
            &FakeTypeMapper,
        );
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_add_head_case_containe() {
        use crate::customizable::property_part_generator::Convertor;
        let space = "// this is comment";
        let mut acc = String::from("id:usize");
        let tobe = format!("{}\n{}", space, acc);
        let mut add_header_convertor = AddHeaderConvertor::new(space);
        add_header_convertor.add_match_property_key("id");
        let dummy_type_name = TypeName::from("");
        let type_key = PropertyKey::from("id");
        let dummy_property_type = make_usize_type();
        add_header_convertor.convert(
            &mut acc,
            &dummy_type_name,
            &type_key,
            &dummy_property_type,
            &FakeTypeMapper,
        );
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_add_right_side_case_all() {
        use crate::customizable::property_part_generator::Convertor;
        let new_line = ",\n";
        let mut acc = String::from("id:usize");
        let tobe = format!("{}{}", acc, new_line);
        let mut add_header_convertor = AddRightSideConvertor::new(new_line);
        add_header_convertor.set_all();
        let dummy_type_name = TypeName::from("");
        let type_key = PropertyKey::from("id");
        let dummy_property_type = make_usize_type();
        add_header_convertor.convert(
            &mut acc,
            &dummy_type_name,
            &type_key,
            &dummy_property_type,
            &FakeTypeMapper,
        );
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_add_left_side_case_all() {
        use crate::customizable::property_part_generator::Convertor;
        let space = "    ";
        let mut acc = String::from("id:usize");
        let tobe = format!("{}{}", space, acc);
        let mut add_header_convertor = AddLeftSideConvertor::new(space);
        add_header_convertor.set_all();
        let dummy_type_name = TypeName::from("");
        let type_key = PropertyKey::from("id");
        let dummy_property_type = make_usize_type();
        add_header_convertor.convert(
            &mut acc,
            &dummy_type_name,
            &type_key,
            &dummy_property_type,
            &FakeTypeMapper,
        );
        assert_eq!(acc, tobe);
    }
    #[test]
    fn test_add_left_side_case_containe() {
        use crate::customizable::property_part_generator::Convertor;
        let space = "    ";
        let mut acc = String::from("id:usize");
        let tobe = format!("{}{}", space, acc);
        let mut add_header_convertor = AddLeftSideConvertor::new(space);
        add_header_convertor.add_match_property_key("id");
        let dummy_type_name = TypeName::from("");
        let type_key = PropertyKey::from("id");
        let dummy_property_type = make_usize_type();
        add_header_convertor.convert(
            &mut acc,
            &dummy_type_name,
            &type_key,
            &dummy_property_type,
            &FakeTypeMapper,
        );
        assert_eq!(acc, tobe);
    }
}
