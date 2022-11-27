use super::util_fns::mkdir;

pub struct TypeGenDistDirectoriesMaker<'a> {
    src: &'a str,
    dist: &'a str,
}

impl<'a> TypeGenDistDirectoriesMaker<'a> {
    pub fn new(src: &'a str, dist: &'a str) -> Self {
        Self { src, dist }
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
