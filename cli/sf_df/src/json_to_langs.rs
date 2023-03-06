use std::path::Path;

use description_generator::{
    type_description_generator::{
        DeclarePartGenerator, PropertyPartGenerator, TypeDescriptionGenerator,
    },
    type_mapper::TypeMapper,
};
use go::description_generator::{
    declare_part_generator::GoDeclarePartGenerator, mapper::GoMapper,
    property_part_generator::GoPropertyPartGenerator, GoTypeDescriptionGenerator,
};
use json::json::Json;
use npc::fns::to_pascal;
use rust::description_generator::{
    declare_part_generator::RustDeclarePartGenerator, mapper::RustMapper,
    property_part_generator::RustPropertyPartGenerator, RustTypeDescriptionGenerator,
};

use crate::{
    configs::FileToFileConfig,
    extension::Extension,
    fileconvertor::{FileStructer, FileStructerConvertor, PathStructure},
    fileoperator::{all_file_structure, file_structures_to_files},
};

pub type JsonToRustConvertor =
    JsonToLangConvertor<RustDeclarePartGenerator, RustPropertyPartGenerator, RustMapper>;
pub type JsonToGoConvertor =
    JsonToLangConvertor<GoDeclarePartGenerator, GoPropertyPartGenerator, GoMapper>;

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

pub fn json_to_rust(config: FileToFileConfig, generator: RustTypeDescriptionGenerator) {
    let src = &config.src;
    let dist = &config.dist;
    let sources = all_file_structure(src, "json");
    let convertor = JsonToRustConvertor::new(src, generator);
    let dists = sources
        .iter()
        .map(|s| convertor.convert(dist, s, "rs").to_snake_path())
        .collect();
    file_structures_to_files(&dists);
    create_rust_mod_files(dist);
}
pub fn json_to_rust_(
    source: impl AsRef<Path>,
    dist: &str,
    generator: RustTypeDescriptionGenerator,
) {
    let convertor = JsonToRustConvertor::new("./", generator);
    let source = FileStructer::from_path(source);
    let result = convertor.convert(dist, &source, "rs").to_snake_path();
    file_structures_to_files(&vec![result]);
}

pub fn json_to_go(source: impl AsRef<Path>, dist: &str, generator: GoTypeDescriptionGenerator) {
    let convertor = JsonToGoConvertor::new("./", generator);
    let source = FileStructer::from_path(source);
    let result = convertor.convert(dist, &source, "go").to_snake_path();
    file_structures_to_files(&vec![result]);
}
pub fn json_dirs_to_go(config: FileToFileConfig, generator: GoTypeDescriptionGenerator) {
    let src = &config.src;
    let dist = &config.dist;
    let sources = all_file_structure(src, "json");
    let convertor = JsonToGoConvertor::new(src, generator);
    let dists = sources
        .iter()
        .map(|s| convertor.convert(dist, s, "go").to_snake_path())
        .collect();
    file_structures_to_files(&dists);
}
// src/parts
// src/parts/nests/data.rs
// src/parts/nests/child.rs
//
// 1 ルートからlsしてdirの名前を.rsにする
// 2 上で作成した.rsの中身をルートとして1をやるって感じか？
// 新規作成によってタイミングがズレるのでぼつ
// src/parts.rs->pub mod nests;
// src/parts/nests.rs->pub mod child;pub mod data;
// src/parts/nests/data.rs
// src/parts/nests/child.rs
//

pub fn create_rust_mod_files(root: &str) {
    fn prepare_parent_files(root: &str) {
        let root_path: &Path = root.as_ref();
        match std::fs::read_dir(root_path) {
            Ok(root_dir) => {
                root_dir
                    .filter_map(|entry| entry.ok())
                    .filter_map(|entry| match entry.file_type() {
                        Ok(file_type) => Some((file_type, entry.path())),
                        Err(_) => None,
                    })
                    .for_each(|(file_type, path)| {
                        if file_type.is_dir() {
                            prepare_parent_files(path.to_str().unwrap());
                        }
                    });

                FileStructer::new(
                    "",
                    PathStructure::new(Extension::to_filepath(root, "rs"), "rs"),
                )
                .new_file();
            }
            Err(e) => panic!("root {} {:#?}", root, e),
        };
    }
    prepare_parent_files(root);
    let mut this_dirs_files = Vec::new();
    let root_path: &Path = root.as_ref();
    match std::fs::read_dir(root_path) {
        Ok(root_dir) => {
            root_dir
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| match entry.file_type() {
                    Ok(file_type) => Some((file_type, entry.path())),
                    Err(_) => None,
                })
                .for_each(|(file_type, path)| {
                    if file_type.is_dir() {
                        create_rust_mod_files(path.to_str().unwrap());
                    } else {
                        this_dirs_files.push(path);
                    }
                });

            FileStructer::new(
                this_dirs_files.into_iter().fold(String::new(), |acc, s| {
                    format!("{}pub mod {};\n", acc, Extension::remove_extension(s))
                }),
                PathStructure::new(Extension::to_filepath(root, "rs"), "rs"),
            )
            .new_file();
        }
        Err(e) => panic!("root {} {:#?}", root, e),
    };
}
