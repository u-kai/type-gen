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
    //
    let config = r#"
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
    let config_file = "tests/type-config.json";
    write_new_file(config_file, config);
    let source_files = [
        "tests/json/parent.json",
        "tests/json/child/child.json",
        "tests/json/child/grand_child/grand_child.json",
    ];
    mkdir_from_filepath(&source_files);
    delete_dirs("tests/json");
    delete_file(config_file)
    //let src_file1=
    // set up source of json

    //json_to_rust_define("tests/type-define-config.json")
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
