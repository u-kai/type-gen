use std::cell::RefCell;

use super::{mapper::LangTypeMapper, type_define_generator::PropertyStatementGenerator};

type PropertyKeyConvertClouser<M: LangTypeMapper> = Box<
    dyn FnMut(
        &mut String,
        &crate::types::type_name::TypeName,
        &crate::types::property_key::PropertyKey,
        &crate::types::property_type::PropertyType,
        &M,
    ) -> (),
>;
type PropertyTypeConvertClouser<M: LangTypeMapper> = PropertyKeyConvertClouser<M>;
pub struct CustomizablePropertyStatementGenerator<M>
where
    M: LangTypeMapper,
{
    property_key_convertor: RefCell<Vec<PropertyKeyConvertClouser<M>>>,
    property_type_convertor: RefCell<Vec<PropertyTypeConvertClouser<M>>>,
}

impl<M> CustomizablePropertyStatementGenerator<M>
where
    M: LangTypeMapper,
{
    pub fn new() -> Self {
        Self {
            property_key_convertor: RefCell::new(Vec::new()),
            property_type_convertor: RefCell::new(Vec::new()),
        }
    }
    pub fn add_property_type_convertor(&self, convertor: PropertyTypeConvertClouser<M>) {
        self.property_type_convertor.borrow_mut().push(convertor);
    }
    pub fn add_property_key_convertor(&self, convertor: PropertyKeyConvertClouser<M>) {
        self.property_key_convertor.borrow_mut().push(convertor);
    }
    fn gen_key_str(
        &self,
        type_name: &crate::types::type_name::TypeName,
        property_key: &crate::types::property_key::PropertyKey,
        property_type: &crate::types::property_type::PropertyType,
        mapper: &M,
    ) -> String {
        let mut key_str = property_key.as_str().to_string();
        self.property_key_convertor
            .borrow_mut()
            .iter_mut()
            .for_each(|c| c(&mut key_str, type_name, property_key, property_type, mapper));
        key_str
    }
    fn gen_type_str(
        &self,
        type_name: &crate::types::type_name::TypeName,
        property_key: &crate::types::property_key::PropertyKey,
        property_type: &crate::types::property_type::PropertyType,
        mapper: &M,
    ) -> String {
        let mut type_str = mapper.case_property_type(property_type);
        self.property_type_convertor
            .borrow_mut()
            .iter_mut()
            .for_each(|c| {
                c(
                    &mut type_str,
                    type_name,
                    property_key,
                    property_type,
                    mapper,
                );
            });
        type_str
    }
}

impl<M> PropertyStatementGenerator<M> for CustomizablePropertyStatementGenerator<M>
where
    M: LangTypeMapper,
{
    fn generate(
        &self,
        type_name: &crate::types::type_name::TypeName,
        property_key: &crate::types::property_key::PropertyKey,
        property_type: &crate::types::property_type::PropertyType,
        mapper: &M,
    ) -> String {
        format!(
            "{}:{}",
            self.gen_key_str(type_name, property_key, property_type, mapper),
            self.gen_type_str(type_name, property_key, property_type, mapper)
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        type_defines::generators::mapper::fake_mapper::FakeLangTypeMapper,
        types::{
            primitive_type::primitive_type_factories::{make_string, make_usize},
            property_key::PropertyKey,
            property_type::{
                property_type_factories::{
                    make_any, make_array_type, make_custom_type, make_primitive_type,
                },
                PropertyType,
            },
            type_name::TypeName,
        },
    };
    #[test]
    fn test_case_primitive_type_none_condition() {
        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_primitive_type(make_usize());
        let tobe = "id:usize";
        let generator = CustomizablePropertyStatementGenerator::new();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );

        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "name".into();
        let property_type: PropertyType = make_primitive_type(make_string());
        let tobe = "name:String";
        let generator = CustomizablePropertyStatementGenerator::new();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );

        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "any".into();
        let property_type: PropertyType = make_any();
        let tobe = "any:any";
        let generator = CustomizablePropertyStatementGenerator::new();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
    #[test]
    fn test_case_composite_type_none_condition() {
        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "obj".into();
        let property_type: PropertyType = make_custom_type("TestObj");
        let tobe = "obj:TestObj";
        let generator = CustomizablePropertyStatementGenerator::new();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
    #[test]
    fn test_case_array_type_none_condition() {
        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_array_type(make_primitive_type(make_usize()));
        let tobe = "id:Vec<usize>";
        let generator = CustomizablePropertyStatementGenerator::new();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );

        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "obj".into();
        let property_type: PropertyType = make_custom_type("TestObj");
        let tobe = "obj:TestObj";
        let generator = CustomizablePropertyStatementGenerator::new();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
    #[test]
    fn test_case_one_convertor_property_type() {
        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_primitive_type(make_usize());
        let optional_checker = |acc: &mut String,
                                type_name: &TypeName,
                                property_key: &PropertyKey,
                                _: &PropertyType,
                                mapper: &FakeLangTypeMapper|
         -> () {
            if type_name.as_str() == "Test" && property_key.as_str() == "id" {
                *acc = mapper.case_optional_type(acc.clone())
            }
        };
        let tobe = "id:Option<usize>";
        let generator = CustomizablePropertyStatementGenerator::new();
        generator.add_property_type_convertor(Box::new(optional_checker));
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
    #[test]
    fn test_case_one_convertor_property_key() {
        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_primitive_type(make_usize());
        let insert_space = |acc: &mut String,
                            _: &TypeName,
                            _: &PropertyKey,
                            _: &PropertyType,
                            _: &FakeLangTypeMapper|
         -> () { *acc = format!("    {}", &acc) };
        let tobe = "    id:usize";
        let generator = CustomizablePropertyStatementGenerator::new();
        generator.add_property_key_convertor(Box::new(insert_space));
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
    #[test]
    fn test_case_multi_convertor_property_type() {
        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_primitive_type(make_usize());
        let optional_checker = |acc: &mut String,
                                type_name: &TypeName,
                                property_key: &PropertyKey,
                                _: &PropertyType,
                                mapper: &FakeLangTypeMapper|
         -> () {
            if type_name.as_str() == "Test" && property_key.as_str() == "id" {
                *acc = mapper.case_optional_type(acc.clone())
            }
        };
        let insert_empty = |acc: &mut String,
                            _: &TypeName,
                            _: &PropertyKey,
                            _: &PropertyType,
                            _: &FakeLangTypeMapper|
         -> () { *acc = format!(" {}", &acc) };
        let tobe = "id: Option<usize>";
        let generator = CustomizablePropertyStatementGenerator::new();
        generator.add_property_type_convertor(Box::new(optional_checker));
        generator.add_property_type_convertor(Box::new(insert_empty));
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
}
