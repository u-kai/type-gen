use std::cell::RefCell;

use crate::types::{primitive_type::PrimitiveType, property_type::PropertyType};

use super::{mapper::LangTypeMapper, type_define_generator::PropertyStatementGenerator};

pub trait ConvertorPropertyKeyStr {
    type Mapper: LangTypeMapper;
    fn call(
        self,
        type_name: &crate::types::type_name::TypeName,
        property_key: &crate::types::property_key::PropertyKey,
        property_type: &crate::types::property_type::PropertyType,
        mapper: &Self::Mapper,
    ) -> String;
}
type F<M: LangTypeMapper> = Box<
    dyn FnMut(
        &mut String,
        &crate::types::type_name::TypeName,
        &crate::types::property_key::PropertyKey,
        &crate::types::property_type::PropertyType,
        &M,
    ) -> (),
>;
pub struct CustomizablePropertyStatementGenerator<M>
where
    //C: FnOnce() -> String,
    M: LangTypeMapper,
{
    property_type_convertor: RefCell<Vec<F<M>>>,
}

impl<M> CustomizablePropertyStatementGenerator<M>
where
    M: LangTypeMapper,
    //    C: ConvertorPropertyKeyStr,
{
    fn new() -> Self {
        Self {
            property_type_convertor: RefCell::new(Vec::new()),
        }
    }
    fn add_property_type_convertor(&mut self, convertor: F<M>) {
        self.property_type_convertor.borrow_mut().push(convertor);
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
        let mut property_key_str = property_key.as_str();
        format!("{}:{}", property_key_str, type_str)
    }
}

#[cfg(test)]
mod test {
    use std::any::type_name;

    use super::*;
    use crate::{
        type_defines::generators::mapper::fake_mapper::FakeLangTypeMapper,
        types::{
            primitive_type::primitive_type_factories::{make_string, make_usize},
            property_key::{self, PropertyKey},
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
                                property_type: &PropertyType,
                                mapper: &FakeLangTypeMapper|
         -> () {
            if type_name.as_str() == "Test" && property_key.as_str() == "id" {
                *acc = mapper.case_optional_type(acc.clone())
            }
        };
        let tobe = "id:Option<usize>";
        let mut generator = CustomizablePropertyStatementGenerator::new();
        generator.add_property_type_convertor(Box::new(optional_checker));
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
                                property_type: &PropertyType,
                                mapper: &FakeLangTypeMapper|
         -> () {
            if type_name.as_str() == "Test" && property_key.as_str() == "id" {
                *acc = mapper.case_optional_type(acc.clone())
            }
        };
        let insert_empty = |acc: &mut String,
                            _: &TypeName,
                            _: &PropertyKey,
                            property_type: &PropertyType,
                            mapper: &FakeLangTypeMapper|
         -> () { *acc = format!(" {}", &acc) };
        let tobe = "id: Option<usize>";
        let mut generator = CustomizablePropertyStatementGenerator::new();
        generator.add_property_type_convertor(Box::new(optional_checker));
        generator.add_property_type_convertor(Box::new(insert_empty));
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
}
