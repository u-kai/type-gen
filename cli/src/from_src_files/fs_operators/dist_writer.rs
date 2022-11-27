use super::{extension::Extension, src_paths::SrcPaths, util_fns::extract_dir};

pub struct TypeDefineDistFilesWriter<'a> {
    src: &'a SrcPaths<'a>,
    dist: &'a str,
    dist_extension: Extension,
}

impl<'a> TypeDefineDistFilesWriter<'a> {
    pub fn new(src: &'a SrcPaths<'a>, dist: &'a str, dist_extension: Extension) -> Self {
        Self {
            dist_extension,
            src,
            dist,
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
                .replace(extension, dist_extension);
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

#[cfg(test)]
mod test_dist {
    use crate::from_src_files::fs_operators::{extension::Extension, src_paths::SrcPaths};

    use super::TypeDefineDistFilesWriter;

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
        let writer = TypeDefineDistFilesWriter::new(&src, "dist", Extension::Rs);
        let mut dists = writer.gen_all_dist_filepath();
        assert_eq!(dists.next().unwrap(), "./dist/test.rs".to_string());
        assert_eq!(dists.next().unwrap(), "./dist/dir1/test.rs".to_string());
        assert_eq!(dists.next().unwrap(), "./dist/dir2/test.rs".to_string());
        assert_eq!(dists.next(), None);
    }
}
