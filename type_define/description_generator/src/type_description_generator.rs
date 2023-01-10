use structure::{
    alias_type_structure::AliasTypeStructure,
    composite_type_structure::CompositeTypeStructure,
    parts::{property_key::PropertyKey, property_type::PropertyType, type_name::TypeName},
    type_structure::TypeStructure,
};

use crate::type_mapper::TypeMapper;

/// TypeDescription is lang type define description string
///
/// like below
/// ```ignore
/// // composite type define description
/// let composite_type:TypeDescription =
/// r#"struct Human {
///     pub name:HumanName,
///     age:usize,
/// }"#.to_string();
///
/// //alias type define description
/// let alias_type:TypeDescription = r#"type HumanName = String;"#.to_string();
/// ```
pub type TypeDescription = String;
/// made lang type define descriptions
/// ```ignore
///
/// pub struct Test {
///     id: usize,
///     child: TestChild
/// }
/// struct TestChild {
///     name: String
///     age: Option<usize>
/// }
/// ```
///
pub struct TypeDescriptionGenerator<Declare, Property, Mapper>
where
    Declare: DeclarePartGenerator<Mapper = Mapper>,
    Property: PropertyPartGenerator<Mapper>,
    Mapper: TypeMapper,
{
    declare_part_generator: Declare,
    property_part_generator: Property,
    mapper: Mapper,
}

/// made case composite
/// ```ignore
/// "struct TypeName { $properties_statement }";
/// ```
pub trait DeclarePartGenerator {
    const TYPE_PREFIX: &'static str = "struct";
    type Mapper: TypeMapper;
    fn generate_case_composite(
        &self,
        composite_type: &CompositeTypeStructure,
        properties_statement: String,
    ) -> String;

    fn generate_case_alias(&self, alias_type: &AliasTypeStructure, mapper: &Self::Mapper)
        -> String;
}

/// made case composite
/// ```ignore
/// "id: usize"
/// ```
/// made case alias
/// ```ignore
/// "type Alias = String;"
/// ```
pub trait PropertyPartGenerator<M>
where
    M: TypeMapper,
{
    fn generate(
        &self,
        type_name: &TypeName,
        property_key: &PropertyKey,
        property_type: &PropertyType,
        mapper: &M,
    ) -> String;
}

impl<Declare, Property, Mapper> TypeDescriptionGenerator<Declare, Property, Mapper>
where
    Declare: DeclarePartGenerator<Mapper = Mapper>,
    Property: PropertyPartGenerator<Mapper>,
    Mapper: TypeMapper,
{
    pub fn new(
        declare_part_generator: Declare,
        property_part_generator: Property,
        mapper: Mapper,
    ) -> Self {
        Self {
            declare_part_generator,
            property_part_generator,
            mapper,
        }
    }
    pub fn generate_concat_define(&self, structures: Vec<TypeStructure>) -> TypeDescription {
        self.generate(structures)
            .into_iter()
            .reduce(|acc, cur| format!("{}\n{}\n", acc, cur))
            .unwrap()
    }
    pub fn generate(&self, structures: Vec<TypeStructure>) -> Vec<TypeDescription> {
        structures
            .into_iter()
            .map(|s| self.generate_one(s))
            .collect()
    }
    pub fn generate_one(&self, structure: TypeStructure) -> TypeDescription {
        match structure {
            TypeStructure::Composite(composite) => {
                let properties_statement =
                    composite
                        .iter()
                        .fold(String::new(), |acc, (property_key, property_type)| {
                            let property_statement = self.property_part_generator.generate(
                                &composite.type_name(),
                                property_key,
                                property_type,
                                &self.mapper,
                            );
                            format!("{}{}", acc, property_statement)
                        });
                self.declare_part_generator
                    .generate_case_composite(&composite, properties_statement)
            }
            TypeStructure::Alias(primitive) => self
                .declare_part_generator
                .generate_case_alias(&primitive, &self.mapper),
        }
    }
}

#[cfg(test)]
pub mod fakes {
    use crate::type_mapper::fake_mapper::FakeTypeMapper;

    use super::*;
    pub struct FakePropertyPartGenerator;
    impl<M> PropertyPartGenerator<M> for FakePropertyPartGenerator
    where
        M: TypeMapper,
    {
        fn generate(
            &self,
            _: &TypeName,
            property_key: &PropertyKey,
            property_type: &PropertyType,
            mapper: &M,
        ) -> String {
            format!(
                "{}: {},",
                property_key.as_str(),
                mapper.case_property_type(property_type)
            )
        }
    }
    pub struct FakeTypeDescriptionGenerator;
    impl DeclarePartGenerator for FakeTypeDescriptionGenerator {
        type Mapper = FakeTypeMapper;
        const TYPE_PREFIX: &'static str = "struct";
        fn generate_case_composite(
            &self,
            compsite_type: &CompositeTypeStructure,
            properties_statement: String,
        ) -> String {
            format!(
                "struct {} {{{}}}",
                compsite_type.type_name().as_str(),
                properties_statement
            )
        }
        fn generate_case_alias(
            &self,
            alias_type: &AliasTypeStructure,
            mapper: &Self::Mapper,
        ) -> String {
            format!(
                "type {} = {};",
                alias_type.type_name().as_str(),
                mapper.case_property_type(&alias_type.property_type())
            )
        }
    }
    #[cfg(test)]
    impl
        TypeDescriptionGenerator<
            FakeTypeDescriptionGenerator,
            FakePropertyPartGenerator,
            FakeTypeMapper,
        >
    {
        pub fn fake_new() -> Self {
            let mapper = FakeTypeMapper;
            Self {
                mapper,
                declare_part_generator: FakeTypeDescriptionGenerator,
                property_part_generator: FakePropertyPartGenerator,
            }
        }
    }
}
#[cfg(test)]

mod test_type_define_statement_generator {
    use structure::{
        parts::property_type::property_type_factories::{
            make_array_type, make_custom_type, make_optional_type, make_string_type,
            make_usize_type,
        },
        type_structure::TypeStructure,
    };

    use crate::type_description_generator::TypeDescriptionGenerator;

    #[test]
    fn test_case_primitive() {
        let simple_statement = TypeStructure::make_alias("Test", make_string_type());
        let tobe = "type Test = String;".to_string();
        let generator = TypeDescriptionGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_optional_case() {
        let simple_statement = TypeStructure::make_composite(
            "Test",
            vec![
                ("id", make_optional_type(make_usize_type())),
                (
                    "child",
                    make_array_type(make_optional_type(make_custom_type("Child"))),
                ),
            ],
        );
        let tobe = "struct Test {child: Vec<Option<Child>>,id: Option<usize>,}".to_string();
        let generator = TypeDescriptionGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_nest_array_case() {
        let simple_statement = TypeStructure::make_composite(
            "Test",
            vec![
                ("id", make_usize_type()),
                (
                    "child",
                    make_array_type(make_array_type(make_custom_type("Child"))),
                ),
            ],
        );
        let tobe = "struct Test {child: Vec<Vec<Child>>,id: usize,}".to_string();
        let generator = TypeDescriptionGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_array_case() {
        let simple_statement = TypeStructure::make_composite(
            "Test",
            vec![
                ("id", make_usize_type()),
                ("child", make_array_type(make_custom_type("Child"))),
            ],
        );
        let tobe = "struct Test {child: Vec<Child>,id: usize,}".to_string();
        let generator = TypeDescriptionGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_has_child_case() {
        let simple_statement = TypeStructure::make_composite(
            "Test",
            vec![
                ("id", make_usize_type()),
                ("child", make_custom_type("Child")),
            ],
        );
        let tobe = "struct Test {child: Child,id: usize,}".to_string();
        let generator = TypeDescriptionGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
        assert_eq!(statements, tobe);
    }
    #[test]
    fn test_simple_case() {
        let simple_statement =
            TypeStructure::make_composite("Test", vec![("id", make_usize_type())]);
        let tobe = "struct Test {id: usize,}".to_string();
        let generator = TypeDescriptionGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
        assert_eq!(statements, tobe);
        let simple_statement = TypeStructure::make_composite(
            "Test",
            vec![("id", make_usize_type()), ("name", make_string_type())],
        );
        let tobe = "struct Test {id: usize,name: String,}".to_string();
        let generator = TypeDescriptionGenerator::fake_new();
        let statements = generator.generate_one(simple_statement);
        assert_eq!(statements, tobe);
    }
}
