use std::{collections::BTreeSet, path::Path};

use super::util_fns::{extract_dir, mkdir};

pub struct TypeDefineDistDirectoriesMaker<'a> {
    src: &'a str,
    dist: &'a str,
}

impl<'a> TypeDefineDistDirectoriesMaker<'a> {
    pub fn new(src: &'a str, dist: &'a str) -> Self {
        Self { src, dist }
    }
    fn gen_all_dist_dir_path<P: AsRef<Path>>(
        &self,
        all_src_files: &'a Vec<P>,
    ) -> impl Iterator<Item = String> + '_ {
        all_src_files
            .iter()
            .filter_map(|p| extract_dir(p))
            .map(|dir| dir.replace(self.src, self.dist))
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
    //pub fn create_dist_directories(&self, all_src_files: &Vec<PathBuf>) {
    //self.generate_all_dist_directory_path(all_src_files)
    //.for_each(|dir| mkdir(dir));
    //}
    //fn generate_all_dist_directory_path(
    //&self,
    //all_src_files: &Vec<PathBuf>,
    //) -> impl Iterator<Item = String> + '_ {
    //all_src_files
    //.into_iter()
    //.map(|src| extract_directory_part_from_path(src).replace(self.src, self.dist))
    //.flat_map(|dist| split_dirs(dist))
    //.into_iter()
    //.collect::<BTreeSet<_>>()
    //.into_iter()
    //}
}

#[cfg(test)]
mod test_dist_dir_maker {
    use super::TypeDefineDistDirectoriesMaker;

    #[test]
    fn test_gen_all_dist_dir_path() {
        let maker = TypeDefineDistDirectoriesMaker::new("src", "dist");
        let all_src_paths = vec![
            "./src/test.txt",
            "./src/dir1/test.txt",
            "./src/dir2/test.txt",
        ];
        let mut dist_dirs = maker.gen_all_dist_dir_path(&all_src_paths);
        assert_eq!(dist_dirs.next().unwrap(), "./dist/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir1/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir2/");
        assert_eq!(dist_dirs.next(), None);
        let all_src_paths = vec![
            "./src/test.txt",
            "./src/test2.txt",
            "./src/dir1/test.txt",
            "./src/dir2/test.txt",
        ];
        let mut dist_dirs = maker.gen_all_dist_dir_path(&all_src_paths);
        assert_eq!(dist_dirs.next().unwrap(), "./dist/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir1/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir2/");
        assert_eq!(dist_dirs.next(), None);
        let all_src_paths = vec![
            "./src/test.txt",
            "./src/dir1/test.txt",
            "./src/dir2/test.txt",
            "./src/dir2/dir3/dir4/dir5/test.txt",
        ];
        let mut dist_dirs = maker.gen_all_dist_dir_path(&all_src_paths);
        assert_eq!(dist_dirs.next().unwrap(), "./dist/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir1/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir2/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir2/dir3/dir4/dir5/");
        assert_eq!(dist_dirs.next(), None);
    }
}
