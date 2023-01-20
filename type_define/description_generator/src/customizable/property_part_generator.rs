use structure::parts::{
    property_key::PropertyKey, property_type::PropertyType, type_name::TypeName,
};

use crate::{type_description_generator::PropertyPartGenerator, type_mapper::TypeMapper};

pub trait Convertor<M: TypeMapper> {
    fn convert(
        &self,
        acc: &mut String,
        type_name: &TypeName,
        property_key: &PropertyKey,
        property_type: &PropertyType,
        mapper: &M,
    ) -> ();
}
pub trait DescriptionConvertor<M: TypeMapper> {
    fn convert(
        &self,
        acc: Option<String>,
        type_name: &TypeName,
        property_key: &PropertyKey,
        property_type: &PropertyType,
        mapper: &M,
    ) -> Option<String>;
}
type PropertyKeyConvertor<M> = Box<dyn Convertor<M>>;
type PropertyTypeConvertor<M> = PropertyKeyConvertor<M>;
pub struct CustomizablePropertyDescriptionGenerator<F, M>
where
    F: Fn(String, String) -> String,
    M: TypeMapper,
{
    concat_key_and_property_type_clouser: F,
    statement_convertor: Vec<Box<dyn DescriptionConvertor<M>>>,
    property_key_convertor: Vec<PropertyKeyConvertor<M>>,
    property_type_convertor: Vec<PropertyTypeConvertor<M>>,
}

impl<F, M> CustomizablePropertyDescriptionGenerator<F, M>
where
    F: Fn(String, String) -> String,
    M: TypeMapper,
{
    pub fn new(f: F) -> Self {
        Self {
            concat_key_and_property_type_clouser: f,
            statement_convertor: Vec::new(),
            property_key_convertor: Vec::new(),
            property_type_convertor: Vec::new(),
        }
    }
    pub fn add_statement_convertor(&mut self, convertor: Box<dyn DescriptionConvertor<M>>) {
        self.statement_convertor.push(convertor);
    }
    pub fn add_property_type_convertor(&mut self, convertor: PropertyTypeConvertor<M>) {
        self.property_type_convertor.push(convertor);
    }
    pub fn add_property_key_convertor(&mut self, convertor: PropertyKeyConvertor<M>) {
        self.property_key_convertor.push(convertor);
    }
    fn gen_key_str(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
        property_type: &PropertyType,
        mapper: &M,
    ) -> String {
        let mut key_str = property_key.as_str().to_string();
        self.property_key_convertor
            .iter()
            .for_each(|c| c.convert(&mut key_str, type_name, property_key, property_type, mapper));
        key_str
    }
    fn gen_type_str(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
        property_type: &PropertyType,
        mapper: &M,
    ) -> String {
        let mut type_str = mapper.case_property_type(property_type);
        self.property_type_convertor.iter().for_each(|c| {
            c.convert(
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

impl<F, M> PropertyPartGenerator<M> for CustomizablePropertyDescriptionGenerator<F, M>
where
    F: Fn(String, String) -> String,
    M: TypeMapper,
{
    fn generate(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
        property_type: &PropertyType,
        mapper: &M,
    ) -> String {
        let c = &self.concat_key_and_property_type_clouser;
        let concated_str = c(
            self.gen_key_str(type_name, property_key, property_type, mapper),
            self.gen_type_str(type_name, property_key, property_type, mapper),
        );
        self.statement_convertor
            .iter()
            .fold(Some(concated_str), |acc, cur| {
                cur.convert(acc, type_name, property_key, property_type, mapper)
            })
            .unwrap_or_default()
    }
}
impl<M> Default for CustomizablePropertyDescriptionGenerator<fn(String, String) -> String, M>
where
    M: TypeMapper,
{
    fn default() -> Self {
        fn default_concat_property_key_and_property_type(
            key_statement: String,
            type_statement: String,
        ) -> String {
            format!("{}:{}", key_statement, type_statement)
        }
        Self {
            concat_key_and_property_type_clouser: default_concat_property_key_and_property_type,
            statement_convertor: Vec::new(),
            property_key_convertor: Vec::new(),
            property_type_convertor: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {

    use structure::parts::property_type::property_type_factories::{
        make_any, make_array_type, make_custom_type, make_string_type, make_usize_type,
    };

    use crate::type_mapper::fake_mapper::FakeTypeMapper;

    use super::*;
    #[test]
    fn test_case_multi_convertor() {
        struct Store<'a> {
            filter: Vec<&'a str>,
        }
        impl<'a> DescriptionConvertor<FakeTypeMapper> for Store<'a> {
            fn convert(
                &self,
                acc: Option<String>,
                _: &TypeName,
                property_key: &PropertyKey,
                _: &PropertyType,
                _mapper: &FakeTypeMapper,
            ) -> Option<String> {
                if self.filter.contains(&property_key.as_str()) {
                    None
                } else {
                    acc
                }
            }
        }
        impl<'a> Store<'a> {
            fn new(v: Vec<&'a str>) -> Self {
                Self { filter: v }
            }
        }
        let mapper = FakeTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_usize_type();
        let store = Store::new(vec!["id", "name"]);
        let tobe = format!("");
        let mut generator = CustomizablePropertyDescriptionGenerator::default();
        generator.add_statement_convertor(Box::new(store));
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe
        );
    }
    #[test]
    fn test_case_add_comment_and_attr() {
        let mapper = FakeTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_usize_type();
        let tobe = format!("// this is comment1\n// this is comment2\n#[cfg(test)]\nid:usize");
        let mut generator = CustomizablePropertyDescriptionGenerator::default();
        struct Clo1 {}
        impl DescriptionConvertor<FakeTypeMapper> for Clo1 {
            fn convert(
                &self,
                acc: Option<String>,
                _: &TypeName,
                _: &PropertyKey,
                _: &PropertyType,
                _: &FakeTypeMapper,
            ) -> Option<String> {
                let add_comment = "// this is comment1\n";
                if let Some(acc) = acc {
                    Some(format!("{}{}", add_comment, acc))
                } else {
                    None
                }
            }
        }
        struct Clo2 {}
        impl DescriptionConvertor<FakeTypeMapper> for Clo2 {
            fn convert(
                &self,
                acc: Option<String>,
                _: &TypeName,
                _: &PropertyKey,
                _: &PropertyType,
                _: &FakeTypeMapper,
            ) -> Option<String> {
                let add_comment = "// this is comment2\n";
                if let Some(acc) = acc {
                    Some(format!("{}{}", add_comment, acc))
                } else {
                    None
                }
            }
        }
        struct Clo3 {}
        impl DescriptionConvertor<FakeTypeMapper> for Clo3 {
            fn convert(
                &self,
                acc: Option<String>,
                _: &TypeName,
                _: &PropertyKey,
                _: &PropertyType,
                _: &FakeTypeMapper,
            ) -> Option<String> {
                let add_comment = "#[cfg(test)]\n";
                if let Some(acc) = acc {
                    Some(format!("{}{}", add_comment, acc))
                } else {
                    None
                }
            }
        }
        generator.add_statement_convertor(Box::new(Clo3 {}));
        generator.add_statement_convertor(Box::new(Clo2 {}));
        generator.add_statement_convertor(Box::new(Clo1 {}));
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe
        );
    }
    #[test]
    fn test_case_primitive_type_concat_split_char_change() {
        let mapper = FakeTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_usize_type();
        let tobe = "id usize";
        let concat_fn = |key: String, type_: String| -> String { format!("{} {}", key, type_) };
        let generator = CustomizablePropertyDescriptionGenerator::new(concat_fn);
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
    #[test]
    fn test_case_primitive_type_none_condition() {
        let mapper = FakeTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_usize_type();
        let tobe = "id:usize";
        let generator = CustomizablePropertyDescriptionGenerator::default();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );

        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "name".into();
        let property_type: PropertyType = make_string_type();
        let tobe = "name:String";
        let generator = CustomizablePropertyDescriptionGenerator::default();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );

        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "any".into();
        let property_type: PropertyType = make_any();
        let tobe = "any:any";
        let generator = CustomizablePropertyDescriptionGenerator::default();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
    #[test]
    fn test_case_composite_type_none_condition() {
        let mapper = FakeTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "obj".into();
        let property_type: PropertyType = make_custom_type("TestObj");
        let tobe = "obj:TestObj";
        let generator = CustomizablePropertyDescriptionGenerator::default();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
    #[test]
    fn test_case_array_type_none_condition() {
        let mapper = FakeTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_array_type(make_usize_type());
        let tobe = "id:Vec<usize>";
        let generator = CustomizablePropertyDescriptionGenerator::default();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );

        let mapper = FakeTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "obj".into();
        let property_type: PropertyType = make_custom_type("TestObj");
        let tobe = "obj:TestObj";
        let generator = CustomizablePropertyDescriptionGenerator::default();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
}
