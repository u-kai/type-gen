use std::{
    fs::File,
    io::{BufWriter, Write},
};

use json::json::Json;
use lang_common::type_defines::{
    additional_defines::{
        additional_statement::AdditionalStatement, attribute_store::Attribute,
        comment_store::Comment, visibility_store::Visibility,
    },
    generators::{
        mapper::LangTypeMapper,
        type_define_generator::{
            PropertyStatementGenerator, TypeDefineGenerator, TypeStatementGenerator,
        },
    },
};

use super::{
    dist_dir_maker::TypeDefineDistDirectoriesMaker, extension::Extension, src_paths::SrcPaths,
    src_reader::TypeDefineSrcReader, util_fns::extract_dir,
};

pub trait TypeDefineDistFileDetail {
    fn convert_lang_filename(&self, original: String) -> String;
    fn add_content(&self, content: String) -> String;
    fn finaly(&self, dist_file: String, writed_content: String);
}
pub struct TypeDefineDistFileWriter<'a> {
    src: &'a SrcPaths<'a>,
    dist: &'a str,
    dist_extension: Extension,
}

impl<'a> TypeDefineDistFileWriter<'a> {
    pub fn new(src: &'a SrcPaths<'a>, dist: &'a str, dist_extension: Extension) -> Self {
        Self {
            dist_extension,
            src,
            dist,
        }
    }
    pub fn write_all_from_jsons<T, P, M, A, V, C, At, D>(
        &self,
        reader: TypeDefineSrcReader,
        type_define_generator: TypeDefineGenerator<T, P, M>,
        detail: D,
    ) where
        T: TypeStatementGenerator,
        P: PropertyStatementGenerator<M>,
        M: LangTypeMapper,
        A: AdditionalStatement,
        V: Visibility,
        C: Comment,
        At: Attribute,
        D: TypeDefineDistFileDetail,
    {
        // setup dist directories
        TypeDefineDistDirectoriesMaker::new(self.src, self.dist).make_dist_dirs();
        //

        let mut all_dist_file = self.gen_all_dist_filepath();
        for src in reader.read_all_srcs() {
            let json = Json::from(src.content());
            let type_structures = json.into_type_structures(src.extracted_filename().unwrap());
            let type_define = type_define_generator.generate_concat_define(type_structures);
            let filename = detail.convert_lang_filename(all_dist_file.next().unwrap());
            let dist_file = File::create(&filename).expect(&format!("not found {}", filename));
            let write_content = detail.add_content(type_define);
            let mut writer = BufWriter::new(&dist_file);
            writer.write_all(write_content.as_bytes()).unwrap();
            detail.finaly(filename, write_content)
        }
    }
    fn gen_all_dist_filepath(&self) -> impl Iterator<Item = String> + '_ {
        self.src.all_src().iter().map(|src_path| {
            let extension = src_path.extension().unwrap().to_str().unwrap();
            let dist_extension: &str = self.dist_extension.into();
            let new_filename = src_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(&format!(".{}", extension), &format!(".{}", dist_extension));
            format!(
                "{}{}",
                extract_dir(src_path)
                    .unwrap()
                    .replace(self.src.src(), self.dist),
                new_filename
            )
        })
    }
}

#[cfg(not(target_os = "windows"))]
#[cfg(test)]
mod test_dist {
    use crate::from_src_files::fs_operators::{
        dist_writer::TypeDefineDistFileWriter, extension::Extension, src_paths::SrcPaths,
    };

    #[test]
    fn test_gen_all_dist_filepath() {
        let src = SrcPaths::for_test(
            "src",
            vec![
                "./src/test.txt",
                "./src/dir1/test.txt",
                "./src/dir2/test.txt",
            ],
        );
        let writer = TypeDefineDistFileWriter::new(&src, "dist", Extension::Rs);
        let mut dists = writer.gen_all_dist_filepath();
        assert_eq!(dists.next().unwrap(), "./dist/test.rs".to_string());
        assert_eq!(dists.next().unwrap(), "./dist/dir1/test.rs".to_string());
        assert_eq!(dists.next().unwrap(), "./dist/dir2/test.rs".to_string());
        assert_eq!(dists.next(), None);
        let src = SrcPaths::for_test(
            "src",
            vec![
                "./src/test.json",
                "./src/dir1/test.json",
                "./src/dir2/json.json",
            ],
        );
        let writer = TypeDefineDistFileWriter::new(&src, "dist", Extension::Rs);
        let mut dists = writer.gen_all_dist_filepath();
        assert_eq!(dists.next().unwrap(), "./dist/test.rs".to_string());
        assert_eq!(dists.next().unwrap(), "./dist/dir1/test.rs".to_string());
        assert_eq!(dists.next().unwrap(), "./dist/dir2/json.rs".to_string());
        assert_eq!(dists.next(), None);
    }
}
