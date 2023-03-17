use rust::generator_builder::RustTypeDescriptionGeneratorBuilder;
use sf_df::{configs::FileToFileConfig, json_to_langs::json_to_rust};

fn main() {
    let config = FileToFileConfig::from_file("config.json").unwrap();
    let generator = RustTypeDescriptionGeneratorBuilder::new()
        .declare_part_all_comment("this is auto generate")
        .declare_part_set_all_derive_with_serde(vec!["Debug", "Clone"])
        .declare_part_pub_all()
        .property_part_all_optional()
        .property_part_pub_all()
        .build();

    json_to_rust(&config.src, &config.dist, generator);
}
