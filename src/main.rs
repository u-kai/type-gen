use type_gen::langs::rust::{
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
                },
                {
                    "id":null,
                    "age":20
                }
            ]
        }
    }
    "#;
    let rust_type_define = RustTypeGeneratorBuilder::new()
        .set_visibility_to_all_struct(RustVisibility::Public)
        .set_visibility_to_all_filed(RustVisibility::PublicSuper)
        .set_attr_to_all_struct(vec![RustTypeAttribute::Derive(vec![
            "Clone".to_string(),
            "Debug".to_string(),
            "Serialize".to_string(),
            "Deserialize".to_string(),
        ])])
        .add_require("UKai", "id")
        .add_comment_to_filed("id", "id is must set")
        .add_comment_to_struct("UKai", "This is Demo")
        .add_comment_to_struct("UKaiProfile", "My Follower is Only One...")
        .build("UKai")
        .gen_from_json(json);
    println!("{}", rust_type_define)
}
