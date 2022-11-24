use cli::type_configuration::ConfigJson;
use json::json::Json;
use langs::rust::builder::RustTypeDefainGeneratorBuilder;

fn main() {
    let config = ConfigJson::from_file("config.json");
    let builder = RustTypeDefainGeneratorBuilder::new();
    let definer = config.to_definer(builder);
    let type_structure = Json::from(r#"{"key":"value"}"#).into_type_structures("Test");
    let statements = definer
        .generate(type_structure)
        .into_iter()
        .reduce(|acc, cur| format!("{}{}\n", acc, cur))
        .unwrap();
    println!("{}", statements);
}
