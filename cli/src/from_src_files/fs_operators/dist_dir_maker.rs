use std::collections::BTreeSet;

use super::{
    src_paths::SrcPaths,
    util_fns::{extract_dir, mkdir_rec},
};

pub struct TypeDefineDistDirectoriesMaker<'a> {
    src: &'a SrcPaths<'a>,
    dist: &'a str,
}

impl<'a> TypeDefineDistDirectoriesMaker<'a> {
    pub fn new(src: &'a SrcPaths<'a>, dist: &'a str) -> Self {
        Self { src, dist }
    }
    pub fn make_dist_dirs(&self) {
        self.gen_all_dist_dir_path()
            .for_each(|dir| mkdir_rec(dir).unwrap());
    }
    fn gen_all_dist_dir_path(&self) -> impl Iterator<Item = String> + '_ {
        self.src
            .all_src()
            .iter()
            .filter_map(|p| extract_dir(p))
            .map(|dir| dir.replace(self.src.src(), self.dist))
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
}

#[cfg(test)]
mod test_dist_dir_maker {
    use crate::from_src_files::fs_operators::src_paths::SrcPaths;

    use super::TypeDefineDistDirectoriesMaker;

    #[test]
    fn test_gen_all_dist_dir_path() {
        let src_path = SrcPaths::for_test(
            "src",
            vec![
                "./src/test.txt",
                "./src/dir1/test.txt",
                "./src/dir2/test.txt",
            ],
        );
        let maker = TypeDefineDistDirectoriesMaker::new(&src_path, "dist");
        let mut dist_dirs = maker.gen_all_dist_dir_path();
        assert_eq!(dist_dirs.next().unwrap(), "./dist/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir1/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir2/");
        assert_eq!(dist_dirs.next(), None);
        let src_path = SrcPaths::for_test(
            "src",
            vec![
                "./src/test.txt",
                "./src/test2.txt",
                "./src/dir1/test.txt",
                "./src/dir2/test.txt",
            ],
        );
        let maker = TypeDefineDistDirectoriesMaker::new(&src_path, "dist");
        let mut dist_dirs = maker.gen_all_dist_dir_path();
        assert_eq!(dist_dirs.next().unwrap(), "./dist/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir1/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir2/");
        assert_eq!(dist_dirs.next(), None);
        let src_path = SrcPaths::for_test(
            "src",
            vec![
                "./src/test.txt",
                "./src/dir1/test.txt",
                "./src/dir2/test.txt",
                "./src/dir2/dir3/dir4/dir5/test.txt",
            ],
        );
        let maker = TypeDefineDistDirectoriesMaker::new(&src_path, "dist");
        let mut dist_dirs = maker.gen_all_dist_dir_path();
        assert_eq!(dist_dirs.next().unwrap(), "./dist/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir1/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir2/");
        assert_eq!(dist_dirs.next().unwrap(), "./dist/dir2/dir3/dir4/dir5/");
        assert_eq!(dist_dirs.next(), None);
    }
}
