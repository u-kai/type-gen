use std::env;

use cli::from_src_files::mains::json_to_rust_define;

fn main() {
    let Some(config_file )= env::args().skip(1).next() else {
        return json_to_rust_define("config.json")
    };
    json_to_rust_define(config_file)
}
