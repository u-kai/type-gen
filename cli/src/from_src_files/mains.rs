use std::path::Path;

use lang_common::type_defines::{
    additional_defines::{
        additional_statement::AdditionalStatement, attribute_store::Attribute,
        comment_store::Comment, visibility_store::Visibility,
    },
    builder::TypeDefineBuilder,
    generators::{
        generator::{PropertyStatementGenerator, TypeStatementGenerator},
        mapper::LangTypeMapper,
    },
};
use langs::rust::builder::RustTypeDefainGeneratorBuilder;

use crate::type_configuration::ConfigJson;

use super::fs_operators::{
    dist_writer::{TypeDefineDistFileDetail, TypeDefineDistFileWriter},
    langs::rust::RustTypeDefineDistFileDetail,
    src_paths::SrcPaths,
    src_reader::TypeDefineSrcReader,
};
pub fn json_to_rust_define(config_file: impl AsRef<Path>) {
    let builder = RustTypeDefainGeneratorBuilder::new();
    let detail = RustTypeDefineDistFileDetail::new();
    json_to_type_define(config_file, builder, detail);
}
fn json_to_type_define<T, P, M, A, V, C, At, D>(
    config_file: impl AsRef<Path>,
    builder: impl TypeDefineBuilder<T, P, M, A, V, C, At>,
    detail: D,
) where
    T: TypeStatementGenerator<M>, //, A>,
    P: PropertyStatementGenerator<M, A>,
    M: LangTypeMapper,
    A: AdditionalStatement,
    V: Visibility,
    C: Comment,
    At: Attribute,
    D: TypeDefineDistFileDetail,
{
    let config = ConfigJson::from_file(config_file);
    let src_paths = SrcPaths::new(config.src());
    let reader = TypeDefineSrcReader::new(&src_paths);
    let writer =
        TypeDefineDistFileWriter::new(&src_paths, config.dist(), config.dist_extension().into());
    let type_define_generator = config.to_definer(builder);
    writer.write_all_from_jsons::<T, P, M, A, V, C, At, D>(reader, type_define_generator, detail);
}
