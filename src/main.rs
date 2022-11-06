use type_gen::rust::{rust_visibility::RustVisibility, type_gen_builder::RustTypeGeneratorBuilder};

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
        .build("UKai")
        .gen_from_json(json);
    println!("{}", rust_type_define)
}
