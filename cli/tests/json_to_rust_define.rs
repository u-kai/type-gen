use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use cli::from_src_files::{
    fs_operators::util_fns::{extract_dir, mkdir_rec},
    mains::json_to_rust_define,
};

#[test]
fn test_json_to_rust_define() {
    //set up config
    let config = TypeDefineConfigFile::default();
    config.write_config_file();
    config.clean_up();
    let source_files = [
        "tests/json/parent.json",
        "tests/json/child/child.json",
        "tests/json/child/grand_child/grand_child.json",
    ];
    mkdir_from_filepath(&source_files);
    delete_dirs("tests/json");
    //let src_file1=
    // set up source of json

    //json_to_rust_define("tests/type-define-config.json")
}

struct TypeDefineConfigFile<'a> {
    config_file: &'a str,
    content: &'a str,
}
impl<'a> TypeDefineConfigFile<'a> {
    fn new(config_file: &'a str, content: &'a str) -> Self {
        Self {
            config_file,
            content,
        }
    }
    fn write_config_file(&self) {
        write_new_file(self.config_file, self.content);
    }
    fn clean_up(self) {
        delete_file(self.config_file)
    }
}
impl<'a> Default for TypeDefineConfigFile<'a> {
    fn default() -> Self {
        let content = r#"
            {
                "src": {
                    "root": "examples/jsons",
                    "extension": "json"
                },
                "dist": {
                    "root": "examples/dist",
                    "extension": "rs"
                }
            }
        "#;
        TypeDefineConfigFile::new("tests/type-define-config.json", content)
    }
}

fn mkdir_from_filepath<P: AsRef<Path>>(paths: &[P]) {
    for path in paths {
        mkdir_rec(extract_dir(path).unwrap()).unwrap()
    }
}
fn write_new_file<P: AsRef<Path>>(path: P, content: &str) {
    let mut config_writer = BufWriter::new(File::create(path).unwrap());
    config_writer.write_all(content.as_bytes()).unwrap();
}

fn delete_dirs<P: AsRef<Path>>(root: P) {
    std::fs::remove_dir_all(root).unwrap()
}
fn delete_file<P: AsRef<Path>>(path: P) {
    std::fs::remove_file(path).unwrap()
}
