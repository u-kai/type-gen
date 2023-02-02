use std::fs::read_to_string;

use description_generator::{
    type_description_generator::{
        DeclarePartGenerator, PropertyPartGenerator, TypeDescriptionGenerator,
    },
    type_mapper::TypeMapper,
};
use json::json::Json;
use npc::fns::to_pascal;
use rust::description_generator::{
    declare_part_generator::RustDeclarePartGenerator, mapper::RustMapper,
    property_part_generator::RustPropertyPartGenerator, RustTypeDescriptionGenerator,
};

use crate::{
    extension::Extension,
    fileconvertor::{FileStructer, FileStructerConvertor, PathStructure},
    fileoperator::{all_file_structure, file_structures_to_files},
};

pub type JsonToRustConvertor =
    JsonToLangConvertor<RustDeclarePartGenerator, RustPropertyPartGenerator, RustMapper>;

pub struct JsonToLangConvertor<Declare, Property, Mapper>
where
    Declare: DeclarePartGenerator<Mapper = Mapper>,
    Property: PropertyPartGenerator<Mapper>,
    Mapper: TypeMapper,
{
    src_root: String,
    generator: TypeDescriptionGenerator<Declare, Property, Mapper>,
}
impl<Declare, Property, Mapper> JsonToLangConvertor<Declare, Property, Mapper>
where
    Declare: DeclarePartGenerator<Mapper = Mapper>,
    Property: PropertyPartGenerator<Mapper>,
    Mapper: TypeMapper,
{
    pub fn new(
        src_root: impl Into<String>,
        generator: TypeDescriptionGenerator<Declare, Property, Mapper>,
    ) -> Self {
        Self {
            src_root: src_root.into(),
            generator,
        }
    }
}
impl<Declare, Property, Mapper> FileStructerConvertor
    for JsonToLangConvertor<Declare, Property, Mapper>
where
    Declare: DeclarePartGenerator<Mapper = Mapper>,
    Property: PropertyPartGenerator<Mapper>,
    Mapper: TypeMapper,
{
    fn convert(
        &self,
        dist_root: &str,
        filestructer: &FileStructer,
        extension: impl Into<Extension>,
    ) -> FileStructer {
        let json = Json::from(filestructer.content());
        let type_structure =
            json.into_type_structures(to_pascal(filestructer.name_without_extension()));
        let rust_type_define = self.generator.generate_concat_define(type_structure);
        filestructer.to_dist(&self.src_root, dist_root, extension, rust_type_define)
    }
}

pub fn json_to_rust(src: &str, dist: &str, generator: RustTypeDescriptionGenerator) {
    let sources = all_file_structure(src, "json");
    let convertor = JsonToRustConvertor::new(src, generator);
    let dists = sources
        .iter()
        .map(|s| convertor.convert(dist, s, "rs").to_snake_path())
        .collect();
    file_structures_to_files(&dists);
    create_rust_mod_file_from_filestructures(&dists);
}

fn create_rust_mod_filestructure(source_file: &FileStructer) -> FileStructer {
    let mut parent = source_file.path().parent_str();
    parent.pop();
    parent.push_str(".rs");
    let path = PathStructure::new(parent, "rs");
    let file_name = source_file.name_without_extension();
    FileStructer::new(format!("pub mod {};", file_name), path)
}
#[test]
fn rustのfile_structureからそのファイルをpub宣言するためのfile_structureを作成する() {
    let source_file = FileStructer::new(
        "pub type St = String;",
        PathStructure::new("./src/parts/types.rs", "rs"),
    );

    let result = create_rust_mod_filestructure(&source_file);

    assert_eq!(
        FileStructer::new("pub mod types;", PathStructure::new("./src/parts.rs", "rs")),
        result
    );

    let source_file = FileStructer::new(
        "pub type St = String;",
        PathStructure::new("./src/parts/data.rs", "rs"),
    );

    let result = create_rust_mod_filestructure(&source_file);

    assert_eq!(
        FileStructer::new("pub mod data;", PathStructure::new("./src/parts.rs", "rs")),
        result
    )
}

// 1. filestructureからそのファイルの名前と親のディレクトリ名をもらう
// 2. 上記のファイル名をpub mod FILE_NAMEとして宣言し，その内容を親のディレクトリ.rsとしてファイル出力する
//
pub fn create_rust_mod_file_from_filestructures(v: &Vec<FileStructer>) {
    v.into_iter()
        .map(|f| create_rust_mod_filestructure(f))
        .for_each(
            |mod_file| match read_to_string(mod_file.path().path_str()) {
                Ok(content) if !content.contains(mod_file.content()) => {
                    mod_file.add_new_line_to_file()
                }
                Err(_) => mod_file.new_file(),
                _ => (),
            },
        )
}
