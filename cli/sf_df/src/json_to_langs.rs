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
    extension::Extension,
    fileconvertor::{FileStructer, FileStructerConvertor, PathStructure},
    fileoperator::{all_file_structure, file_structures_to_files, is_dir},
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

pub fn json_to_rust(src: &str, dist: &str, generator: RustTypeDescriptionGenerator) {
    json_to_lang(src, dist, generator, "rs");
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
    let src = FsType::new(src);
    let dist = FsType::new(dist);
    match src {
        FsType::Dir(src_root) => match dist {
            FsType::Dir(dist_root) => {
                json_dir_to_lang_dir(src_root, dist_root, generator, extension);
            }
            FsType::File(dist_file) => {
                let sources = all_file_structure(src_root, "json");
                let contents = sources
                    .into_iter()
                    .map(|file| {
                        (
                            to_pascal(file.name_without_extension()),
                            Json::from(file.content()),
                        )
                    })
                    .map(|(filename, json)| json.into_type_structures(filename))
                    .map(|type_structures| generator.generate_concat_define(type_structures))
                    .fold(String::new(), |acc, cur| format!("{}{}", acc, cur));
                FileStructer::new(contents, PathStructure::from_path(dist_file)).new_file();
            }
        },
        FsType::File(src_file) => match dist {
            FsType::Dir(dist_root) => {
                let src = FileStructer::from_path(src_file);
                let convertor = JsonToLangConvertor::new(src.path().parent_str(), generator);
                let dist = convertor.convert(dist_root, &src, extension);
                file_structures_to_files(&vec![dist]);
            }
            FsType::File(dist_file) => {
                json_file_to_lang_file(src_file, dist_file, generator, extension);
            }
        },
    }
}

// src の読み込み-> 読み込んだ内容を変換 -> distに配置
// srcがファイルでdistがdirならsrcの内容を変換したものをdistの子供として配置する
// srcがファイルでdistもファイルならsrcの内容を変換したものをそのままdistのファイルとして作成すれば良い
// srcがディレクトリでdistもディレクトリなら,srcのなかのファイル群をすべて読み取り，その親パスをそのままdist二編こすれば良い
// src/test.json src/child/child.json -> dist/test.rs dist/child/child.rs

//fn convert(src: FsType, dist: FsType) -> FsType {
//src
//}
fn convert_json_to_lang(json_root: FsType, lang_root: FsType) {
    let sources = read_json_files(json_root);
}

fn read_json_files(root: FsType) -> Vec<FileStructer> {
    match root {
        FsType::File(file) => vec![FileStructer::from_path(file)],
        FsType::Dir(dir) => all_file_structure(dir, "json"),
    }
}

#[cfg(test)]
mod tests {
    // #[test]
}

enum FsType<'a> {
    File(&'a str),
    Dir(&'a str),
}
impl<'a> FsType<'a> {
    fn new(path: &'a str) -> Self {
        if is_dir(path) {
            Self::Dir(path)
        } else {
            Self::File(path)
        }
    }
}

pub fn json_dir_to_lang_dir<D, P, M>(
    src_root: &str,
    dist_root: &str,
    generator: TypeDescriptionGenerator<D, P, M>,
    extension: impl Into<Extension>,
) where
    D: DeclarePartGenerator<Mapper = M>,
    P: PropertyPartGenerator<M>,
    M: TypeMapper,
{
    // src
    let sources = all_file_structure(src_root, "json");
    //let convertor = JsonToLangConvertor::new(src, generator);
    let extension = extension.into();
    let dists = sources
        .into_iter()
        .map(|src| {
            let json = Json::from(src.content());
            let type_structure = json.into_type_structures(to_pascal(src.name_without_extension()));
            let content = generator.generate_concat_define(type_structure);
            let dist = src.to_dist(src_root, dist_root, extension, content);
            dist.to_snake_path()
        })
        .collect();
    //let dists = sources
    //.iter()
    //.map(|s| convertor.convert(dist, s, extension).to_snake_path())
    //.collect();
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
    let result = convertor.convert(dist, &source, extension).to_snake_path();
    file_structures_to_files(&vec![result]);
}

pub fn json_to_go(src: &str, dist: &str, generator: GoTypeDescriptionGenerator) {
    json_to_lang(src, dist, generator, "go");
}

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
            Err(e) => println!("not dirs {}, {:#?}", root, e),
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
        Err(e) => println!("not dirs {}, {:#?}", root, e),
    };
}
