use structure::parts::{property_key::PropertyKey, type_name::TypeName};

use crate::type_mapper::TypeMapper;

use super::property_part_generator::Convertor;

struct PropertyPartMatchConditionStore<'a> {
    is_all: bool,
    match_type_names: Vec<&'a str>,
    match_propety_keys: Vec<&'a str>,
    match_type_name_and_propety_keys: Vec<(&'a str, &'a str)>,
}
impl<'a> PropertyPartMatchConditionStore<'a> {
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
    fn add_match_type_name(&mut self, type_name: &'a str) {
        self.match_type_names.push(type_name);
    }
    fn add_match_property_key(&mut self, property_key: &'a str) {
        self.match_propety_keys.push(property_key);
    }
    fn add_match_type_name_and_property_key(&mut self, type_name: &'a str, property_key: &'a str) {
        self.match_type_name_and_propety_keys
            .push((type_name, property_key));
    }
    fn is_match(&self, type_name: &'a str, property_key: &'a str) -> bool {
        self.is_all
            || self.match_type_names.contains(&type_name)
            || self.match_propety_keys.contains(&property_key)
            || self
                .match_type_name_and_propety_keys
                .contains(&(type_name, property_key))
    }
}
macro_rules! impl_match_condition_store_methods {
    ($($t:ident)*) => {
        $(impl<'a> $t<'a> {
            pub fn set_all(&mut self) {
                self.store.set_all()
            }
            pub fn add_match_type_name(&mut self, type_name: &'a str) {
                self.store.add_match_type_name(type_name);
            }
            pub fn add_match_property_key(&mut self, property_key: &'a str) {
                self.store.add_match_property_key(property_key);
            }
            pub fn add_match_type_name_and_property_key(&mut self, type_name: &'a str, property_key: &'a str) {
                self.store.add_match_type_name_and_property_key(type_name, property_key);
            }
        })*
    };
}
pub struct AddHeaderConvertor<'a> {
    header: &'a str,
    store: PropertyPartMatchConditionStore<'a>,
}
impl<'a> AddHeaderConvertor<'a> {
    pub fn new(header: &'a str) -> Self {
        Self {
            header,
            store: PropertyPartMatchConditionStore::new(),
        }
    }
}
impl_match_condition_store_methods!(AddHeaderConvertor);

impl<'a, M> Convertor<M> for AddHeaderConvertor<'a>
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
            *acc = format!("{}{}", self.header, acc)
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::type_mapper::fake_mapper::FakeTypeMapper;
    use structure::parts::{
        property_key::PropertyKey, property_type::property_type_factories::make_usize_type,
        type_name::TypeName,
    };

    #[test]
    fn test_add_header_case_all() {
        let space = "    ";
        let mut acc = String::from("id:usize");
        let tobe = format!("{}{}", space, acc);
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
    fn test_add_header_case_containe() {
        let space = "    ";
        let mut acc = String::from("id:usize");
        let tobe = format!("{}{}", space, acc);
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
}
