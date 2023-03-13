use description_generator::{
    customizable::property_part_generator::{Convertor, CustomizablePropertyDescriptionGenerator},
    type_description_generator::PropertyPartGenerator,
};
use npc::fns::to_pascal;

use super::mapper::GoMapper;

pub struct GoPropertyPartGenerator {
    inner: CustomizablePropertyDescriptionGenerator<fn(String, String) -> String, GoMapper>,
}

impl GoPropertyPartGenerator {
    fn concat_fn(type_name: String, type_: String) -> String {
        format!("   {} {}\n", type_name, type_)
    }
    pub fn new() -> Self {
        Self {
            inner: CustomizablePropertyDescriptionGenerator::new(Self::concat_fn),
        }
    }
}

impl PropertyPartGenerator<GoMapper> for GoPropertyPartGenerator {
    fn generate(
        &self,
        type_name: &structure::parts::type_name::TypeName,
        property_key: &structure::parts::property_key::PropertyKey,
        property_type: &structure::parts::property_type::PropertyType,
        mapper: &GoMapper,
    ) -> String {
        self.inner
            .generate(type_name, property_key, property_type, mapper)
    }
}
pub struct GoPropertyPartGeneratorBuilder {
    generator: GoPropertyPartGenerator,
}
impl GoPropertyPartGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            generator: GoPropertyPartGenerator::new(),
        }
    }
    pub fn json_marshal(mut self) -> Self {
        struct AddJsonMarshalConvertor {}
        impl Convertor<GoMapper> for AddJsonMarshalConvertor {
            fn convert(
                &self,
                acc: &mut String,
                _type_name: &structure::parts::type_name::TypeName,
                property_key: &structure::parts::property_key::PropertyKey,
                _property_type: &structure::parts::property_type::PropertyType,
                _mapper: &GoMapper,
            ) -> () {
                if acc.contains("`json:") {
                    return;
                } else {
                    *acc = format!(r#"{} `json:"{}"`"#, acc, property_key.as_str())
                }
            }
        }
        self.generator
            .inner
            .add_property_type_convertor(Box::new(AddJsonMarshalConvertor {}));
        self
    }
    pub fn all_optional(mut self) -> Self {
        struct AddJsonMarshalOptionalConvertor {}
        impl Convertor<GoMapper> for AddJsonMarshalOptionalConvertor {
            fn convert(
                &self,
                acc: &mut String,
                _type_name: &structure::parts::type_name::TypeName,
                property_key: &structure::parts::property_key::PropertyKey,
                _property_type: &structure::parts::property_type::PropertyType,
                _mapper: &GoMapper,
            ) -> () {
                if acc.contains("`json:") {
                    let from = format!(r#"`json:"{}"`"#, property_key.as_str());
                    let to = format!(r#"`json:"{},omitempty"`"#, property_key.as_str());
                    *acc = acc.replace(&from, &to)
                } else {
                    *acc = format!(r#"{} `json:"{},omitempty"`"#, acc, property_key.as_str())
                }
            }
        }
        self.generator
            .inner
            .add_property_type_convertor(Box::new(AddJsonMarshalOptionalConvertor {}));
        self
    }
    pub fn set_whitelist_with_keys(mut self, list: Vec<impl Into<String>>) -> Self {
        self.generator.inner.set_whitelist_with_keys(list);
        self
    }
    pub fn set_blacklist_with_keys(mut self, list: Vec<impl Into<String>>) -> Self {
        self.generator.inner.set_blacklist_with_keys(list);
        self
    }
    pub fn pub_all(mut self) -> Self {
        struct ToPascalConvertor {}
        impl Convertor<GoMapper> for ToPascalConvertor {
            fn convert(
                &self,
                acc: &mut String,
                _type_name: &structure::parts::type_name::TypeName,
                property_key: &structure::parts::property_key::PropertyKey,
                _property_type: &structure::parts::property_type::PropertyType,
                _mapper: &GoMapper,
            ) -> () {
                *acc = format!("{}", to_pascal(property_key.as_str()));
            }
        }
        self.generator
            .inner
            .add_property_key_convertor(Box::new(ToPascalConvertor {}));
        self
    }
    pub fn build(self) -> GoPropertyPartGenerator {
        self.generator
    }
}

#[cfg(test)]
mod tests {
    use description_generator::type_description_generator::PropertyPartGenerator;
    use structure::parts::{
        property_key::PropertyKey, property_type::property_type_factories::make_usize_type,
        type_name::TypeName,
    };

    use crate::description_generator::{
        mapper::GoMapper,
        property_part_generator::{GoPropertyPartGenerator, GoPropertyPartGeneratorBuilder},
    };

    #[test]
    fn optionalの設定だけでjsonの設定も行われる() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_usize_type();
        let mapper = GoMapper;

        let sut = GoPropertyPartGeneratorBuilder::new()
            .pub_all()
            .all_optional()
            .build();
        assert_eq!(
            sut.generate(&type_name, &property_key, &property_type, &mapper,),
            r#"   Id int `json:"id,omitempty"`
"#
        );
    }
    #[test]
    fn 全てのプロパティをoptionalかつパブリックにしてjsonの元の名前を記述する() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_usize_type();
        let mapper = GoMapper;

        let sut = GoPropertyPartGeneratorBuilder::new()
            .pub_all()
            .json_marshal()
            .all_optional()
            .build();
        assert_eq!(
            sut.generate(&type_name, &property_key, &property_type, &mapper,),
            r#"   Id int `json:"id,omitempty"`
"#
        );
    }
    #[test]
    fn 全てのプロパティをパブリックにしてjsonの元の名前を記述する() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_usize_type();
        let mapper = GoMapper;

        let sut = GoPropertyPartGeneratorBuilder::new()
            .pub_all()
            .json_marshal()
            .build();
        assert_eq!(
            sut.generate(&type_name, &property_key, &property_type, &mapper,),
            r#"   Id int `json:"id"`
"#
        );
    }
    #[test]
    fn 全てのプロパティをパブリックにする() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_usize_type();
        let mapper = GoMapper;

        let sut = GoPropertyPartGeneratorBuilder::new().pub_all().build();
        assert_eq!(
            sut.generate(&type_name, &property_key, &property_type, &mapper,),
            "   Id int\n"
        );
    }
    #[test]
    fn idという整数型のプロパティの作成() {
        let type_name: TypeName = "Test".into();
        let property_key: PropertyKey = "id".into();
        let property_type = make_usize_type();
        let mapper = GoMapper;

        let sut = GoPropertyPartGenerator::new();
        assert_eq!(
            sut.generate(&type_name, &property_key, &property_type, &mapper,),
            "   id int\n"
        );
    }
}
