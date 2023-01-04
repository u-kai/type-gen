use crate::types::{property_key::PropertyKey, property_type::PropertyType, type_name::TypeName};

pub mod generator;
pub mod mapper;

type PropertyStatement = String;
type TypeStatement = String;
pub trait PropertyStatementGenerator {
    fn generate_property_statement(
        &self,
        root_name: &TypeName,
        property_key: &PropertyKey,
        property_type: &PropertyType,
    ) -> PropertyStatement;
    fn generate_type_statement(
        &self,
        type_name: &TypeName,
        inner_statement: String,
    ) -> TypeStatement;
}

#[cfg(test)]
mod test {
    use crate::types::{
        primitive_type::primitive_type_factories::make_usize,
        property_type::property_type_factories::make_primitive_type, structures::TypeStructure,
    };

    //    struct FakeStatementGenerator<'a> {
    //        types:Vec<&'a str>,
    //
    //    }
    //    fn test_generate_simple_type() {
    //        let tobe = "struct Test {id: usize,name: String,}".to_string();
    //        let generator = TypeDefineGenerator::new();
    //        let type_structure =
    //            TypeStructure::make_composite("Test", vec![("id", make_primitive_type(make_usize()))]);
    //        assert_eq!(generator.generates(type_structure), tobe);
    //    }
}
