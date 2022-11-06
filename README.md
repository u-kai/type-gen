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
        .set_visibility_to_all_filed(RustVisibility::PublicSuper)
        .build("UKai")
        .gen_from_json(json);
    println!("{}", rust_type_define)
}

// Output below!!
pub struct UKai {
    pub(super) id: Option<i64>,
    pub(super) profile: Option<UKaiProfile>,
}

pub struct UKaiProfile {
    pub(super) follower: Option<Vec<UKaiProfileFollower>>,
    pub(super) name: Option<String>,
    #[serde(rename = "userId")]
    pub(super) user_id: Option<i64>,
}

pub struct UKaiProfileFollower {
    pub(super) name: Option<String>,
}

```
