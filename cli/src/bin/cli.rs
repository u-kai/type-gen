use cli::type_configuration::ConfigJson;
use json::json::Json;
use langs::rust::builder::RustTypeDefainGeneratorBuilder;

fn main() {
    let config = ConfigJson::from_file("config.json");
    config.test();
    let builder = RustTypeDefainGeneratorBuilder::new();
    let definer = config.to_definer(builder);
    let type_structure = Json::from(r#"{"key":"value"}"#).into_type_structures("Test");
    let statements = definer.generate_concat_define(type_structure);
    println!("{}", statements);
}
