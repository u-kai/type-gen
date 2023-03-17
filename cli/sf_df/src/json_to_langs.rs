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
        let type_define = self.generator.generate_concat_define(type_structure);
        filestructer.to_dist(&self.src_root, dist_root, extension, type_define)
    }
}

// source -> dir or file
// dist -> dir or file
// 上記は一致しているべき？
// config を賢くすればいい気もしてきた
// configとしてはdirでもfileでも柔軟に変換の設定をできるようにしたい

pub fn json_to_rust(config: FileToFileConfig, generator: RustTypeDescriptionGenerator) {
    let src = &config.src;
    let dist = &config.dist;
    json_to_lang(src, dist, generator, "rs");
    //let sources = all_file_structure(src, "json");
    //let convertor = JsonToRustConvertor::new(src, generator);
    //let dists = sources
    //.iter()
    //.map(|s| convertor.convert(dist, s, "rs").to_snake_path())
    //.collect();
    //file_structures_to_files(&dists);
    create_rust_mod_files(dist);
}
pub fn json_to_lang<D, P, M>(
    src: &str,
    dist: &str,
    generator: TypeDescriptionGenerator<D, P, M>,
    extension: impl Into<Extension>,
) where
    D: DeclarePartGenerator<Mapper = M>,
    P: PropertyPartGenerator<M>,
    M: TypeMapper,
{
    let src_path: &Path = src.as_ref();
    let extension: Extension = extension.into();
    if src_path.is_file() && extension.is_this_extension(src_path) {
        json_file_to_lang_file(src, dist, generator, extension)
    } else {
        json_dir_to_lang_dir(src, dist, generator, extension)
    }
}

#[test]
fn pathを指定した拡張子とディレクトリに変換する() {
    let src = PathStructure::from_path("test/test.json");
    let dist_parent = "dist";
    let sut = src.to(dist_parent, "rs");

    assert_eq!(sut.path_str(), "dist/test.rs");
}
// srcがファイルでdistがdir指定
// srcがdirでdistがdir
// srcがdirでdistがfile(一ファイルに集約？)

enum Fs<'a> {
    File(&'a str),
    Dir(&'a str),
}
impl<'a> Fs<'a> {
    fn new(path: &'a str, extension: impl Into<Extension>) -> Self {
        let path_: &Path = path.as_ref();
        let extension: Extension = extension.into();
        if path_.is_file() && extension.is_this_extension(path) {
            Self::File(path)
        } else {
            Self::Dir(path)
        }
    }
}

pub fn json_dir_to_lang_dir<D, P, M>(
    src: &str,
    dist: &str,
    generator: TypeDescriptionGenerator<D, P, M>,
    extension: impl Into<Extension>,
) where
    D: DeclarePartGenerator<Mapper = M>,
    P: PropertyPartGenerator<M>,
    M: TypeMapper,
{
    // src
    let sources = all_file_structure(src, "json");
    let convertor = JsonToLangConvertor::new(src, generator);
    let extension: Extension = extension.into();
    let dists = sources
        .iter()
        .map(|s| convertor.convert(dist, s, extension).to_snake_path())
        .collect();
    file_structures_to_files(&dists);
}
pub fn json_file_to_lang_file<D, P, M>(
    src: &str,
    dist: &str,
    generator: TypeDescriptionGenerator<D, P, M>,
    extension: impl Into<Extension>,
) where
    D: DeclarePartGenerator<Mapper = M>,
    P: PropertyPartGenerator<M>,
    M: TypeMapper,
{
    let convertor = JsonToLangConvertor::new("", generator);
    let source = FileStructer::from_path(src);
    println!("{:#?}", source);
    let result = convertor.convert(dist, &source, extension).to_snake_path();
    println!("{:#?}", result);
    file_structures_to_files(&vec![result]);
}
pub fn json_to_rust_(
    source: impl AsRef<Path>,
    dist: &str,
    generator: RustTypeDescriptionGenerator,
) {
    let convertor = JsonToRustConvertor::new("", generator);
    let source = FileStructer::from_path(source);
    let result = convertor.convert(dist, &source, "rs").to_snake_path();
    file_structures_to_files(&vec![result]);
}

pub fn json_to_go(source: impl AsRef<Path>, dist: &str, generator: GoTypeDescriptionGenerator) {
    let source = FileStructer::from_path(source);
    let convertor = JsonToGoConvertor::new("", generator);
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
