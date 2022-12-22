# type-gen

## What is it?

-   type-gen is generate some programing language type define from json, other language type define.

## How to use

-   below sample is json -> rust type define

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
    use std::env;
    use cli::from_src_files::mains::json_to_rust_define;
    fn main() {
        let Some(config_file)= env::args().skip(1).next() else {
            return json_to_rust_define("config.json")
        };
        json_to_rust_define(config_file)
    }

/// ------------------Output below!!-------------------- ///

// This is Demo
use serde::{Serialize,Deserialize};
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
