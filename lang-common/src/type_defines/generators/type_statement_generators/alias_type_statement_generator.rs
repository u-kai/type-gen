use crate::{
    type_defines::generators::mapper::LangTypeMapper,
    types::{structures::AliasTypeStructure, type_name::TypeName},
};

pub trait TypeConvertor<M: LangTypeMapper> {
    fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure, mapper: &M) -> ();
}
pub trait TypeIdentifyConvertor {
    fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> ();
}
pub trait AliasTypeStatementConvertor {
    fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> ();
}
pub struct CustomizableAliasTypeStatementGenerator<M, F>
where
    M: LangTypeMapper,
    F: Fn(&str, &TypeName, String) -> String,
{
    alias_type_identify: &'static str,
    concat_fn: F,
    statement_convertors: Vec<Box<dyn AliasTypeStatementConvertor>>,
    type_convertors: Vec<Box<dyn TypeConvertor<M>>>,
    type_identify_convertors: Vec<Box<dyn TypeIdentifyConvertor>>,
}
impl<M, F> CustomizableAliasTypeStatementGenerator<M, F>
where
    M: LangTypeMapper,
    F: Fn(&str, &TypeName, String) -> String,
{
    pub fn new(alias_type_identify: &'static str, concat_fn: F) -> Self {
        Self {
            alias_type_identify,
            concat_fn,
            statement_convertors: Vec::new(),
            type_convertors: Vec::new(),
            type_identify_convertors: Vec::new(),
        }
    }
    pub fn add_statement_convertor(&mut self, convertor: Box<dyn AliasTypeStatementConvertor>) {
        self.statement_convertors.push(convertor)
    }
    pub fn add_type_convertor(&mut self, convertor: Box<dyn TypeConvertor<M>>) {
        self.type_convertors.push(convertor)
    }
    pub fn add_type_identify_convertor(&mut self, convertor: Box<dyn TypeIdentifyConvertor>) {
        self.type_identify_convertors.push(convertor)
    }
    pub(super) fn generate_type_define(
        &self,
        alias_type: &crate::types::structures::AliasTypeStructure,
        mapper: &M,
    ) -> String {
        let f = &self.concat_fn;
        let type_identify = self.gen_type_identify(alias_type);
        let type_statement = self.gen_type(alias_type, mapper);
        let mut statement = f(&type_identify, &alias_type.name, type_statement);
        self.statement_convertors
            .iter()
            .for_each(|c| c.convert(&mut statement, alias_type));
        statement
    }
    fn gen_type(
        &self,
        alias_type: &crate::types::structures::AliasTypeStructure,
        mapper: &M,
    ) -> String {
        let mut result = mapper.case_property_type(&alias_type.property_type);
        self.type_convertors
            .iter()
            .for_each(|c| c.convert(&mut result, alias_type, mapper));
        result
    }
    fn gen_type_identify(
        &self,
        alias_type: &crate::types::structures::AliasTypeStructure,
    ) -> String {
        let mut result = format!("{}", self.alias_type_identify);
        self.type_identify_convertors
            .iter()
            .for_each(|c| c.convert(&mut result, alias_type));
        result
    }
}
pub(super) fn concat_fn(identify: &str, type_name: &TypeName, statement: String) -> String {
    format!("{} {} = {}", identify, type_name.as_str(), statement)
}
#[cfg(test)]
mod test {
    use crate::{
        type_defines::generators::mapper::fake_mapper::FakeLangTypeMapper,
        types::{
            primitive_type::PrimitiveType, property_type::PropertyType,
            structures::AliasTypeStructure,
        },
    };

    use super::*;
    #[test]
    fn test_alias_type_case_simple() {
        let identify = "interface";
        let type_name: TypeName = "Test".into();
        let tobe = "interface Test = String".to_string();
        let mapper = FakeLangTypeMapper;
        let generator = CustomizableAliasTypeStatementGenerator::new(identify, concat_fn);
        assert_eq!(
            generator.generate_type_define(
                &AliasTypeStructure::new(type_name, PropertyType::Primitive(PrimitiveType::String)),
                &mapper
            ),
            tobe
        );
    }
    #[test]
    fn test_alias_type_case_add_pub_and_optional_condition() {
        let mapper = FakeLangTypeMapper;
        let alias_type_identify = "type";
        let type_name: TypeName = "Test".into();
        struct AddOptionalConvertor {
            targets: Vec<&'static str>,
        }
        impl TypeConvertor<FakeLangTypeMapper> for AddOptionalConvertor {
            fn convert(
                &self,
                acc: &mut String,
                alias_type: &AliasTypeStructure,
                m: &FakeLangTypeMapper,
            ) -> () {
                if self.targets.contains(&alias_type.name.as_str()) {
                    *acc = m.case_optional_type(acc.clone());
                }
            }
        }
        struct AddPubConvertor {
            targets: Vec<&'static str>,
        }
        impl TypeIdentifyConvertor for AddPubConvertor {
            fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
                if self.targets.contains(&alias_type.name.as_str()) {
                    *acc = format!("pub {}", acc);
                }
            }
        }

        let mut generator =
            CustomizableAliasTypeStatementGenerator::new(alias_type_identify, concat_fn);
        let optional_convertor = AddOptionalConvertor {
            targets: vec!["Test", "Id"],
        };
        let pub_convertor = AddPubConvertor {
            targets: vec!["Test", "Id"],
        };
        let pub_convertor = Box::new(pub_convertor);
        generator.add_type_convertor(Box::new(optional_convertor));
        generator.add_type_identify_convertor(pub_convertor);
        let tobe = "pub type Test = Option<String>".to_string();
        assert_eq!(
            generator.generate_type_define(
                &AliasTypeStructure::new(type_name, PropertyType::Primitive(PrimitiveType::String)),
                &mapper
            ),
            tobe
        );
    }
    #[test]
    fn test_alias_type_case_add_comment_and_attr() {
        let mapper = FakeLangTypeMapper;
        let alias_type_identify = "type";
        let type_name: TypeName = "Test".into();
        struct AddAttrConvertor {
            targets: Vec<&'static str>,
        }
        impl AliasTypeStatementConvertor for AddAttrConvertor {
            fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
                if self.targets.contains(&alias_type.name.as_str()) {
                    *acc = format!("#[derive(Debug)]\n{}", acc);
                }
            }
        }
        struct AddCommentConvertor {
            targets: Vec<&'static str>,
        }
        impl AliasTypeStatementConvertor for AddCommentConvertor {
            fn convert(&self, acc: &mut String, alias_type: &AliasTypeStructure) -> () {
                if self.targets.contains(&alias_type.name.as_str()) {
                    *acc = format!("// this is comment\n{}", acc);
                }
            }
        }

        let mut generator =
            CustomizableAliasTypeStatementGenerator::new(alias_type_identify, concat_fn);
        let attr_convertor = AddAttrConvertor {
            targets: vec!["Test", "Id"],
        };
        let comment_convertor = AddCommentConvertor {
            targets: vec!["Test", "Id"],
        };
        let attr_convertor = Box::new(attr_convertor);
        let comment_convertor = Box::new(comment_convertor);
        generator.add_statement_convertor(attr_convertor);
        generator.add_statement_convertor(comment_convertor);
        let tobe = "// this is comment\n#[derive(Debug)]\ntype Test = String".to_string();
        assert_eq!(
            generator.generate_type_define(
                &AliasTypeStructure::new(type_name, PropertyType::Primitive(PrimitiveType::String)),
                &mapper
            ),
            tobe
        );
    }
}
