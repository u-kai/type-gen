use type_gen::rust::{
    rust_visibility::RustVisibility, type_gen_builder::RustTypeGeneratorBuilder,
    type_statements::type_attr::RustTypeAttribute,
};

fn main() {
    let json = r#"
    {
        "id":0,
        "profile":{
            "userId":123,
            "name":"u-kai",
            "follower":[
                {
                    "name":"something"
                }
            ]
        }
    }
    "#;
    let rust_type_define = RustTypeGeneratorBuilder::new()
        .set_visibility_to_all_struct(RustVisibility::Public)
        .set_visibility_to_all_filed(RustVisibility::PublicSuper)
        .set_attr_to_all_struct(vec![
            RustTypeAttribute::Derive(vec![
                "Clone".to_string(),
                "Debug".to_string(),
                "Serialize".to_string(),
                "Deserialize".to_string(),
            ]),
            RustTypeAttribute::CfgTest,
        ])
        .build("UKai")
        .gen_from_json(json);
    println!("{}", rust_type_define)
}
