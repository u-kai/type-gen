fn main() {}
//use std::path::Path;

//use cli::{
//file_operators::{Extension, TypeDefineSrcReader, TypeGenDistFilesWriter},
//type_configuration::ConfigJson,
//};
//use lang_common::type_defines::{
//additional_defines::{
//additional_statement::AdditionalStatement, attribute_store::Attribute,
//comment_store::Comment, visibility_store::Visibility,
//},
//builder::TypeDefineBuilder,
//generators::{
//generator::{PropertyStatementGenerator, TypeStatementGenerator},
//mapper::LangTypeMapper,
//},
//};
//use langs::rust::builder::RustTypeDefainGeneratorBuilder;

//fn main() {
//let builder = RustTypeDefainGeneratorBuilder::new();
//jsons_to_type_define("config.json", builder);
//}

//fn jsons_to_type_define<T, P, M, A, V, C, At>(
//config_file: impl AsRef<Path>,
//builder: impl TypeDefineBuilder<T, P, M, A, V, C, At>,
//) where
//T: TypeStatementGenerator<M, A>,
//P: PropertyStatementGenerator<M, A>,
//M: LangTypeMapper,
//A: AdditionalStatement,
//V: Visibility,
//C: Comment,
//At: Attribute,
//{
//let config = ConfigJson::from_file(config_file);
//let reader = TypeDefineSrcReader::new(config.src());
//let writer = TypeGenDistFilesWriter::new(
//config.src(),
//config.dist(),
//Extension::from(config.dist_extension()),
//);
//let type_structure_define = config.to_definer(builder);
//writer.write_all_from_jsons::<T, P, M, A, V, C, At>(reader, type_structure_define);
//}
