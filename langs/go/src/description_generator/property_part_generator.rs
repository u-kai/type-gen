use description_generator::{
    customizable::property_part_generator::CustomizablePropertyDescriptionGenerator,
    type_description_generator::PropertyPartGenerator,
};

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

#[cfg(test)]
mod tests {
    use description_generator::type_description_generator::PropertyPartGenerator;
    use structure::parts::{
        property_key::PropertyKey, property_type::property_type_factories::make_usize_type,
        type_name::TypeName,
    };

    use crate::description_generator::{
        mapper::GoMapper, property_part_generator::GoPropertyPartGenerator,
    };

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
