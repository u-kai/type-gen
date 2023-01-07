use super::{mapper::LangTypeMapper, type_define_generator::PropertyStatementGenerator};

pub trait Convertor<M: LangTypeMapper> {
    fn convert(
        &self,
        acc: &mut String,
        type_name: &crate::types::type_name::TypeName,
        property_key: &crate::types::property_key::PropertyKey,
        property_type: &crate::types::property_type::PropertyType,
        mapper: &M,
    ) -> ();
}
type PropertyKeyConvertor<M: LangTypeMapper> = Box<dyn Convertor<M>>;
type PropertyTypeConvertor<M: LangTypeMapper> = PropertyKeyConvertor<M>;
type StatementConvertor<M: LangTypeMapper> = PropertyKeyConvertor<M>;
pub struct CustomizablePropertyStatementGenerator<F, M>
where
    F: Fn(String, String) -> String,
    M: LangTypeMapper,
{
    concut_key_and_property_type_clouser: F,
    statement_convertor: Vec<StatementConvertor<M>>,
    property_key_convertor: Vec<PropertyKeyConvertor<M>>,
    property_type_convertor: Vec<PropertyTypeConvertor<M>>,
}

impl<F, M> CustomizablePropertyStatementGenerator<F, M>
where
    F: Fn(String, String) -> String,
    M: LangTypeMapper,
{
    pub fn new(f: F) -> Self {
        Self {
            concut_key_and_property_type_clouser: f,
            statement_convertor: Vec::new(),
            property_key_convertor: Vec::new(),
            property_type_convertor: Vec::new(),
        }
    }
    pub fn add_statement_convertor(&mut self, convertor: StatementConvertor<M>) {
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
        type_name: &crate::types::type_name::TypeName,
        property_key: &crate::types::property_key::PropertyKey,
        property_type: &crate::types::property_type::PropertyType,
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
        type_name: &crate::types::type_name::TypeName,
        property_key: &crate::types::property_key::PropertyKey,
        property_type: &crate::types::property_type::PropertyType,
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

impl<F, M> PropertyStatementGenerator<M> for CustomizablePropertyStatementGenerator<F, M>
where
    F: Fn(String, String) -> String,
    M: LangTypeMapper,
{
    fn generate(
        &self,
        type_name: &crate::types::type_name::TypeName,
        property_key: &crate::types::property_key::PropertyKey,
        property_type: &crate::types::property_type::PropertyType,
        mapper: &M,
    ) -> String {
        let c = &self.concut_key_and_property_type_clouser;
        let mut concated_str = c(
            self.gen_key_str(type_name, property_key, property_type, mapper),
            self.gen_type_str(type_name, property_key, property_type, mapper),
        );
        self.statement_convertor.iter().for_each(|c| {
            c.convert(
                &mut concated_str,
                type_name,
                property_key,
                property_type,
                mapper,
            );
        });
        concated_str
    }
}
impl<M> Default for CustomizablePropertyStatementGenerator<fn(String, String) -> String, M>
where
    M: LangTypeMapper,
{
    fn default() -> Self {
        fn default_concat_property_key_and_property_type(
            key_statement: String,
            type_statement: String,
        ) -> String {
            format!("{}:{}", key_statement, type_statement)
        }
        Self {
            concut_key_and_property_type_clouser: default_concat_property_key_and_property_type,
            statement_convertor: Vec::new(),
            property_key_convertor: Vec::new(),
            property_type_convertor: Vec::new(),
        }
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
    fn test_case_multi_convertor() {
        struct Store<'a> {
            filter: Vec<&'a str>,
        }
        impl<'a> Convertor<FakeLangTypeMapper> for Store<'a> {
            fn convert(
                &self,
                acc: &mut String,
                _: &crate::types::type_name::TypeName,
                property_key: &crate::types::property_key::PropertyKey,
                _: &crate::types::property_type::PropertyType,
                _mapper: &FakeLangTypeMapper,
            ) -> () {
                if self.filter.contains(&property_key.as_str()) {
                    *acc = "".to_string();
                }
            }
        }
        impl<'a> Store<'a> {
            fn new(v: Vec<&'a str>) -> Self {
                Self { filter: v }
            }
        }
        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_primitive_type(make_usize());
        let store = Store::new(vec!["id", "name"]);
        let tobe = format!("");
        let mut generator = CustomizablePropertyStatementGenerator::default();
        generator.add_statement_convertor(Box::new(store));
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe
        );
    }
    #[test]
    fn test_case_add_comment_and_attr() {
        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_primitive_type(make_usize());
        let tobe = format!("// this is comment1\n// this is comment2\n#[cfg(test)]\nid:usize");
        let mut generator = CustomizablePropertyStatementGenerator::default();
        struct Clo1 {}
        impl Convertor<FakeLangTypeMapper> for Clo1 {
            fn convert(
                &self,
                acc: &mut String,
                _: &crate::types::type_name::TypeName,
                _: &crate::types::property_key::PropertyKey,
                _: &crate::types::property_type::PropertyType,
                _: &FakeLangTypeMapper,
            ) -> () {
                let add_comment = "// this is comment1\n";
                *acc = format!("{}{}", add_comment, acc);
            }
        }
        struct Clo2 {}
        impl Convertor<FakeLangTypeMapper> for Clo2 {
            fn convert(
                &self,
                acc: &mut String,
                _: &crate::types::type_name::TypeName,
                _: &crate::types::property_key::PropertyKey,
                _: &crate::types::property_type::PropertyType,
                _: &FakeLangTypeMapper,
            ) -> () {
                let add_comment = "// this is comment2\n";
                *acc = format!("{}{}", add_comment, acc);
            }
        }
        struct Clo3 {}
        impl Convertor<FakeLangTypeMapper> for Clo3 {
            fn convert(
                &self,
                acc: &mut String,
                _: &crate::types::type_name::TypeName,
                _: &crate::types::property_key::PropertyKey,
                _: &crate::types::property_type::PropertyType,
                _: &FakeLangTypeMapper,
            ) -> () {
                let add_comment = "#[cfg(test)]\n";
                *acc = format!("{}{}", add_comment, acc);
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
        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_primitive_type(make_usize());
        let tobe = "id usize";
        let concat_fn = |key: String, type_: String| -> String { format!("{} {}", key, type_) };
        let generator = CustomizablePropertyStatementGenerator::new(concat_fn);
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
    #[test]
    fn test_case_primitive_type_none_condition() {
        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type: PropertyType = make_primitive_type(make_usize());
        let tobe = "id:usize";
        let generator = CustomizablePropertyStatementGenerator::default();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );

        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "name".into();
        let property_type: PropertyType = make_primitive_type(make_string());
        let tobe = "name:String";
        let generator = CustomizablePropertyStatementGenerator::default();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );

        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "any".into();
        let property_type: PropertyType = make_any();
        let tobe = "any:any";
        let generator = CustomizablePropertyStatementGenerator::default();
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
        let generator = CustomizablePropertyStatementGenerator::default();
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
        let generator = CustomizablePropertyStatementGenerator::default();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );

        let mapper = FakeLangTypeMapper;
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "obj".into();
        let property_type: PropertyType = make_custom_type("TestObj");
        let tobe = "obj:TestObj";
        let generator = CustomizablePropertyStatementGenerator::default();
        assert_eq!(
            generator.generate(&type_name, &property_key, &property_type, &mapper),
            tobe.to_string()
        );
    }
    //    #[test]
    //    fn test_case_one_convertor_property_type() {
    //        let mapper = FakeLangTypeMapper;
    //        let type_name: TypeName = "Test".into();
    //        let property_key: PropertyKey = "id".into();
    //        let property_type: PropertyType = make_primitive_type(make_usize());
    //        let optional_checker = |acc: &mut String,
    //                                type_name: &TypeName,
    //                                property_key: &PropertyKey,
    //                                _: &PropertyType,
    //                                mapper: &FakeLangTypeMapper|
    //         -> () {
    //            if type_name.as_str() == "Test" && property_key.as_str() == "id" {
    //                *acc = mapper.case_optional_type(acc.clone())
    //            }
    //        };
    //        let tobe = "id:Option<usize>";
    //        let generator = CustomizablePropertyStatementGenerator::default();
    //        generator.add_property_type_convertor(Box::new(optional_checker));
    //        assert_eq!(
    //            generator.generate(&type_name, &property_key, &property_type, &mapper),
    //            tobe.to_string()
    //        );
    //    }
    //    #[test]
    //    fn test_case_one_convertor_property_key() {
    //        let mapper = FakeLangTypeMapper;
    //        let type_name: TypeName = "Test".into();
    //        let property_key: PropertyKey = "id".into();
    //        let property_type: PropertyType = make_primitive_type(make_usize());
    //        let insert_space = |acc: &mut String,
    //                            _: &TypeName,
    //                            _: &PropertyKey,
    //                            _: &PropertyType,
    //                            _: &FakeLangTypeMapper|
    //         -> () { *acc = format!("    {}", &acc) };
    //        let tobe = "    id:usize";
    //        let generator = CustomizablePropertyStatementGenerator::default();
    //        generator.add_property_key_convertor(Box::new(insert_space));
    //        assert_eq!(
    //            generator.generate(&type_name, &property_key, &property_type, &mapper),
    //            tobe.to_string()
    //        );
    //    }
    //    #[test]
    //    fn test_case_multi_convertor_property_key() {
    //        let mapper = FakeLangTypeMapper;
    //        let type_name: TypeName = "Test".into();
    //        let property_key: PropertyKey = "id".into();
    //        let property_type: PropertyType = make_primitive_type(make_usize());
    //        let insert_pub = |acc: &mut String,
    //                          _: &TypeName,
    //                          _: &PropertyKey,
    //                          _: &PropertyType,
    //                          _: &FakeLangTypeMapper|
    //         -> () { *acc = format!("pub {}", &acc) };
    //        let insert_space = |acc: &mut String,
    //                            _: &TypeName,
    //                            _: &PropertyKey,
    //                            _: &PropertyType,
    //                            _: &FakeLangTypeMapper|
    //         -> () { *acc = format!("    {}", &acc) };
    //        let tobe = "    pub id:usize";
    //        let generator = CustomizablePropertyStatementGenerator::default();
    //        generator.add_property_key_convertor(Box::new(insert_pub));
    //        generator.add_property_key_convertor(Box::new(insert_space));
    //        assert_eq!(
    //            generator.generate(&type_name, &property_key, &property_type, &mapper),
    //            tobe.to_string()
    //        );
    //    }
    //    #[test]
    //    fn test_case_multi_convertor_property_type() {
    //        let mapper = FakeLangTypeMapper;
    //        let type_name: TypeName = "Test".into();
    //        let property_key: PropertyKey = "id".into();
    //        let property_type: PropertyType = make_primitive_type(make_usize());
    //        let optional_checker = |acc: &mut String,
    //                                type_name: &TypeName,
    //                                property_key: &PropertyKey,
    //                                _: &PropertyType,
    //                                mapper: &FakeLangTypeMapper|
    //         -> () {
    //            if type_name.as_str() == "Test" && property_key.as_str() == "id" {
    //                *acc = mapper.case_optional_type(acc.clone())
    //            }
    //        };
    //        let insert_empty = |acc: &mut String,
    //                            _: &TypeName,
    //                            _: &PropertyKey,
    //                            _: &PropertyType,
    //                            _: &FakeLangTypeMapper|
    //         -> () { *acc = format!(" {}", &acc) };
    //        let tobe = "id: Option<usize>";
    //        let generator = CustomizablePropertyStatementGenerator::default();
    //        generator.add_property_type_convertor(Box::new(optional_checker));
    //        generator.add_property_type_convertor(Box::new(insert_empty));
    //        assert_eq!(
    //            generator.generate(&type_name, &property_key, &property_type, &mapper),
    //            tobe.to_string()
    //        );
    //    }
}
