use std::{
    collections::BTreeSet,
    fs::{read_to_string, File, OpenOptions},
    io::Write,
    path::Path,
};

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
pub fn create_rust_mod_file_from_filestructures(root_dir: &str, v: &Vec<FileStructer>) {
    fn get_parent_filename(path: &str) -> Option<String> {
        fn get_writed_filename(dist_file: impl AsRef<Path>) -> Option<String> {
            Some(dist_file.as_ref().file_name()?.to_str()?.to_string())
        }
        let path: &Path = path.as_ref();
        Some(
            path.to_str()?
                .replace(&format!("{}{}", "/", get_writed_filename(path)?), ".rs"),
        )
    }
    fn f(filestructure: &FileStructer) {
        let mod_name: &Path = filestructure.path().path_str().as_ref();
        let mod_name = mod_name
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".rs", "");
        println!("mod name = {}", mod_name);
        if let Some(parent_filename) = get_parent_filename(filestructure.path().path_str()) {
            let path: &Path = parent_filename.as_ref();
            println!("path = {:#?}", path);
            let mut file = if path.exists() {
                OpenOptions::new()
                    .append(true)
                    .read(true)
                    .open(path)
                    .expect(&format!(
                        "path is {:?}, mod_name {} , \ncontent {}",
                        path,
                        mod_name,
                        filestructure.content()
                    ))
            } else {
                File::create(path).expect(&format!(
                    "path is {:?}, mod_name {} , \ncontent {}",
                    path,
                    mod_name,
                    filestructure.content()
                ))
            };
            let write_mod_content = format!("pub mod {};\n", mod_name);
            match read_to_string(path) {
                Ok(str) if !str.contains(&write_mod_content) => {
                    file.write_all(write_mod_content.as_bytes())
                        .expect(&format!(
                            "path is {:?}, mod_name {} , \ncontent {}",
                            path,
                            mod_name,
                            filestructure.content()
                        ));
                }
                Err(e) => println!("{:#?}", e),
                _ => {
                    println!(
                        "not consider case . read_string is {:#?}",
                        read_to_string(path)
                    )
                }
            }
        }
    }
    let child_dirs = v
        .iter()
        .flat_map(|f| f.path().all_child_dirs(root_dir))
        .collect::<BTreeSet<String>>()
        .into_iter()
        .map(|s| PathStructure::new(s, "rs"))
        .map(|p| {
            let parent = p.parent_str();
            format!("{}.rs", &parent[..parent.len() - 1])
        })
        .inspect(|f| println!("{}", f));
}

//impl RustTypeDefineDistFileDetail {
//    pub fn new() -> Self {
//        Self {
//            dependencies: vec!["serde::{Deserialize,Serialize}"],
//        }
//    }
//    fn get_parent_filename(dist_file: impl AsRef<Path>) -> Option<String> {
//    }
//}
//impl TypeDefineDistFileDetail for RustTypeDefineDistFileDetail {
//    fn add_content(&self, content: String) -> String {
//        self.dependencies
//            .iter()
//            .fold(content, |acc, cur| format!("use {};\n{}", cur, acc))
//    }
//    fn convert_lang_filename(&self, original: String) -> String {
//        NamingPrincipalConvertor::new(&original).to_snake()
//    }
//    fn finaly(&self, dist_file: String, writed_content: String) {
//        let mod_name: &Path = dist_file.as_ref();
//        let mod_name = mod_name
//            .file_name()
//            .unwrap()
//            .to_str()
//            .unwrap()
//            .replace(".rs", "");
//        println!("mod name = {}", mod_name);
//        if let Some(parent_filename) = Self::get_parent_filename(&dist_file) {
//            let path: &Path = parent_filename.as_ref();
//            println!("path = {:#?}", path);
//            let mut file = if path.exists() {
//                OpenOptions::new()
//                    .append(true)
//                    .read(true)
//                    .open(path)
//                    .expect(&format!(
//                        "path is {:?}, mod_name {} , \ncontent {}",
//                        path, mod_name, writed_content
//                    ))
//            } else {
//                File::create(path).expect(&format!(
//                    "path is {:?}, mod_name {} , \ncontent {}",
//                    path, mod_name, writed_content
//                ))
//            };
//            let write_mod_content = format!("pub mod {};\n", mod_name);
//            match read_to_string(path) {
//                Ok(str) if !str.contains(&write_mod_content) => {
//                    file.write_all(write_mod_content.as_bytes())
//                        .expect(&format!(
//                            "path is {:?}, mod_name {} , \ncontent {}",
//                            path, mod_name, writed_content
//                        ));
//                }
//                Err(e) => println!("{:#?}", e),
//                _ => {
//                    println!(
//                        "not consider case . read_string is {:#?}",
//                        read_to_string(path)
//                    )
//                }
//            }
//        }
//    }
//}
