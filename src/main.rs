use type_gen::rust::type_gen::RustTypeGenerator;

fn main() {
    let mut rust = RustTypeGenerator::new("Test");
    rust.add_derives("Test", vec!["Debug", "Clone"]);
    rust.add_derives("TestData", vec!["Debug", "Clone"]);
    rust.set_pub_filed("id");
    rust.set_pub_struct("Test");
    let json = r#"
        {
            "data":[
                {
                    "id":12345,
                    "userName":"u-kai",
                    "test":"test-string",
                    "entities":{
                        "id":0
                    }
                }
            ]
        }
    "#;
    let result = rust.from_json_example(json);
    println!("{}", result)
}
