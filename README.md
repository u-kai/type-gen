# type-gen

## What is it?

- type-gen is generate some programing language type define from json, other language type define.

## How to use

- below sample is json -> rust type define

```rust
fn main () {
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
    }"#;
    let rust_type_define = RustTypeGeneratorBuilder::new()
        .set_visibility_to_all_struct(RustVisibility::Public)
        .set_visibility_to_all_field(RustVisibility::PublicSuper)
        .set_attr_to_all_struct(vec![RustTypeAttribute::Derive(vec![
            "Clone".to_string(),
            "Debug".to_string(),
            "Serialize".to_string(),
            "Deserialize".to_string(),
        ])])
        .add_require("UKai", "id")
        .add_comment_to_field("id", "id is must set")
        .add_comment_to_struct("UKai", "This is Demo")
        .add_comment_to_struct("UKaiProfile", "My Follower is Only One...")
        .build("UKai")
        .gen_from_json(json);
    println!("{}", rust_type_define)
}

/// ------------------Output below!!-------------------- ///

// This is Demo
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct UKai {
    // id is must set
    pub(super) id: i64,
    pub(super) profile: Option<UKaiProfile>,
}

// My Follower is Only One...
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct UKaiProfile {
    pub(super) follower: Option<Vec<UKaiProfileFollower>>,
    pub(super) name: Option<String>,
    #[serde(rename = "userId")]
    pub(super) user_id: Option<i64>,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct UKaiProfileFollower {
    pub(super) name: Option<String>,
}
```
