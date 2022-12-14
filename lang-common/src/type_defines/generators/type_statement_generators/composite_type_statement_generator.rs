use crate::types::{structures::CompositeTypeStructure, type_name::TypeName};

pub trait TypeIdentifyConvertor {
    fn convert(&self, acc: &mut String, type_name: &TypeName) -> ();
}
pub trait CompositeTypeStatementConvertor {
    fn convert(&self, acc: &mut String, composite_type: &CompositeTypeStructure) -> ();
}
pub struct CustomizableCompositeTypeStatementGenerator<F>
where
    F: Fn(&str, &TypeName, String) -> String,
{
    type_identify: &'static str,
    concat_fn: F,
    type_identify_convertors: Vec<Box<dyn TypeIdentifyConvertor>>,
    statement_convertors: Vec<Box<dyn CompositeTypeStatementConvertor>>,
}
impl<F> CustomizableCompositeTypeStatementGenerator<F>
where
    F: Fn(&str, &TypeName, String) -> String,
{
    pub fn new(type_identify: &'static str, concat_fn: F) -> Self {
        Self {
            type_identify,
            concat_fn,
            type_identify_convertors: Vec::new(),
            statement_convertors: Vec::new(),
        }
    }
    pub fn add_type_identify_convertor(&mut self, convertor: Box<dyn TypeIdentifyConvertor>) {
        self.type_identify_convertors.push(convertor);
    }
    pub fn add_statement_convertor(&mut self, convertor: Box<dyn CompositeTypeStatementConvertor>) {
        self.statement_convertors.push(convertor);
    }
    pub(super) fn generate_type_define(
        &self,
        composite_type: &CompositeTypeStructure,
        properties_statement: String,
    ) -> String {
        let f = &self.concat_fn;
        let type_name = composite_type.type_name();
        let type_identify = self.gen_type_identify(type_name);
        let mut statement = f(&type_identify, type_name, properties_statement);
        self.statement_convertors
            .iter()
            .for_each(|c| c.convert(&mut statement, composite_type));
        statement
    }
    fn gen_type_identify(&self, type_name: &TypeName) -> String {
        let mut type_identify = self.type_identify.to_string();
        self.type_identify_convertors
            .iter()
            .for_each(|c| c.convert(&mut type_identify, type_name));
        type_identify
    }
}
#[cfg(test)]
pub(super) fn default_concat_fn(
    type_identify: &str,
    type_name: &TypeName,
    property_statements: String,
) -> String {
    format!(
        "{} {} {{{}}}",
        type_identify,
        type_name.as_str(),
        property_statements
    )
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::*;
    #[test]
    fn test_composite_type_add_comment_and_attr() {
        let property_statements = "id:usize".to_string();
        let type_identify = "class";
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        struct AddAttr {}
        impl CompositeTypeStatementConvertor for AddAttr {
            fn convert(&self, acc: &mut String, _: &CompositeTypeStructure) {
                *acc = format!("#[derive(Debug)]\n{}", acc);
            }
        }
        struct AddComment {}
        impl CompositeTypeStatementConvertor for AddComment {
            fn convert(&self, acc: &mut String, _: &CompositeTypeStructure) {
                *acc = format!("// this is comment\n{}", acc);
            }
        }
        let add_attr = Box::new(AddAttr {});
        let add_comment = Box::new(AddComment {});
        let mut generator =
            CustomizableCompositeTypeStatementGenerator::new(type_identify, default_concat_fn);
        generator.add_statement_convertor(add_attr);
        generator.add_statement_convertor(add_comment);
        let tobe = "// this is comment\n#[derive(Debug)]\nclass Test {id:usize}";
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_statements),
            tobe.to_string()
        );

        let type_identify = "class";
        let property_statements = "id:usize".to_string();
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let tobe = "class Test {id:usize}";
        let generator =
            CustomizableCompositeTypeStatementGenerator::new(type_identify, default_concat_fn);
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_statements),
            tobe.to_string()
        )
    }
    #[test]
    fn test_composite_type_add_pub() {
        let property_statements = "id:usize".to_string();
        let type_identify = "struct";
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let tobe = "pub struct Test {id:usize}";
        struct AddPub {}
        impl TypeIdentifyConvertor for AddPub {
            fn convert(&self, acc: &mut String, _: &TypeName) {
                *acc = format!("pub {}", acc);
            }
        }
        let add_pub = Box::new(AddPub {});
        let mut generator =
            CustomizableCompositeTypeStatementGenerator::new(type_identify, default_concat_fn);
        generator.add_type_identify_convertor(add_pub);
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_statements),
            tobe.to_string()
        );
    }
    #[test]
    fn test_case_composite_type_simple() {
        let property_statements = "id:usize".to_string();
        let type_identify = "struct";
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let tobe = "struct Test {id:usize}";
        let generator =
            CustomizableCompositeTypeStatementGenerator::new(type_identify, default_concat_fn);
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_statements),
            tobe.to_string()
        );

        let type_identify = "class";
        let property_statements = "id:usize".to_string();
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        let tobe = "class Test {id:usize}";
        let generator =
            CustomizableCompositeTypeStatementGenerator::new(type_identify, default_concat_fn);
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_statements),
            tobe.to_string()
        )
    }
    #[test]
    fn test_type_define_use_convertor() {
        let property_statements = "    id:usize".to_string();
        let type_identify = "struct";
        let type_name: TypeName = "Test".into();
        let dummy_composite_type = CompositeTypeStructure::new(type_name, BTreeMap::new());
        fn concat_identity_and_name_and_property_statement(
            type_identify: &str,
            type_name: &TypeName,
            property_statements: String,
        ) -> String {
            format!(
                "{} {} {{\n{}\n}}",
                type_identify,
                type_name.as_str(),
                property_statements
            )
        }
        let tobe = "struct Test {
    id:usize
}";
        let generator = CustomizableCompositeTypeStatementGenerator::new(
            type_identify,
            concat_identity_and_name_and_property_statement,
        );
        assert_eq!(
            generator.generate_type_define(&dummy_composite_type, property_statements),
            tobe.to_string()
        );
    }
}
