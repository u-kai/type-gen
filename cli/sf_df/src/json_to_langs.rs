use std::path::Path;

use description_generator::{
    type_description_generator::{
        DeclarePartGenerator, PropertyPartGenerator, TypeDescriptionGenerator,
    },
    type_mapper::TypeMapper,
};
use go::description_generator::GoTypeDescriptionGenerator;
use json::json::Json;
use npc::fns::to_pascal;
use rust::description_generator::RustTypeDescriptionGenerator;

use crate::{
    extension::Extension,
    fileconvertor::{FileStructer, PathStructure},
    fileoperator::{all_file_structure, file_structures_to_files, is_dir},
};

pub fn json_to_rust(src: &str, dist: &str, generator: RustTypeDescriptionGenerator) {
    json_to_lang(src, dist, generator, "rs");
    create_rust_mod_files(dist);
}

pub fn json_to_go(src: &str, dist: &str, generator: GoTypeDescriptionGenerator) {
    json_to_lang(src, dist, generator, "go");
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
                json_file_into_dist(src_file, dist_root, generator, extension);
            }
            FsType::File(dist_file) => {
                json_file_into_dist(src_file, dist_file, generator, extension);
            }
        },
    }
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

fn json_dir_to_lang_dir<D, P, M>(
    src_root: &str,
    dist_root: &str,
    generator: TypeDescriptionGenerator<D, P, M>,
    extension: impl Into<Extension>,
) where
    D: DeclarePartGenerator<Mapper = M>,
    P: PropertyPartGenerator<M>,
    M: TypeMapper,
{
    let sources = all_file_structure(src_root, "json");
    let extension = extension.into();
    let dists = sources
        .into_iter()
        .map(|src| {
            let json = Json::from(src.content());
            let type_structure = json.into_type_structures(to_pascal(src.name_without_extension()));
            let content = generator.generate_concat_define(type_structure);
            let dist = src.to_dist(src_root, dist_root, extension, content);
            dist.to_snake_path_consider_with_wellknown_words()
        })
        .collect();
    file_structures_to_files(&dists);
}
fn json_file_into_dist<D, P, M>(
    src_file: &str,
    dist_root: &str,
    generator: TypeDescriptionGenerator<D, P, M>,
    extension: impl Into<Extension>,
) where
    D: DeclarePartGenerator<Mapper = M>,
    P: PropertyPartGenerator<M>,
    M: TypeMapper,
{
    let src = FileStructer::from_path(src_file);
    let json = Json::from(src.content());
    let type_structure = json.into_type_structures(to_pascal(src.name_without_extension()));
    let content = generator.generate_concat_define(type_structure);
    let dist = src
        .to(dist_root, extension, content)
        .to_snake_path_consider_with_wellknown_words();
    dist.new_file();
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
