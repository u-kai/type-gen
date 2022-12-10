use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use cli::from_src_files::mains::json_to_rust_define;

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
    },
    "comment": {
        "alltype": "this is auto make type",
        "allproperty": "this is auto make property"
    },
    "attribute": {
        "alltype": "allow(unused)\nderive(Serialize, Deserialize,Clone,Debug)",
        "allproperty": "allow(unused)"
    },
    "visibility": {
        "alltype": "pub",
        "allproperty": "pub"
    },
    "optional": {
        "all": true
    }
    }
    "#;
    write_new_file("tests/type-config.json", config);
    let source_dirs = [
        "tests/json",
        "tests/json/child",
        "tests/json/child/grand_child",
    ];
    //let src_file1=
    // set up source of json

    //json_to_rust_define("tests/type-define-config.json")
}

fn write_new_file<P: AsRef<Path>>(path: P, content: &str) {
    let mut config_writer = BufWriter::new(File::create(path).unwrap());
    config_writer.write_all(content.as_bytes()).unwrap();
}
