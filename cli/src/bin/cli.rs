use cli::type_configuration::ConfigJson;
use json::into_type_structure::IntoTypeStructureJson;
use langs::rust::builder::RustTypeDefainGeneratorBuilder;

fn main() {
    let config = ConfigJson::from_file("config.json");
    config.test();
    let builder = RustTypeDefainGeneratorBuilder::new();
    let definer = config.to_definer(builder);
    let type_structure = IntoTypeStructureJson::from_str(r#"{"key":"value"}"#, "Test").into();
    let statements = definer.generate_concat_define(type_structure);
    println!("{}", statements);
}
