# type-gen

## What is it?

- type-gen is generate some programing language type define from json, other language type define.

## How to use

- below sample is json -> rust type define

```rust

// config.json is configure src dirs and dist dirs
//
//  {
//    "src": "examples/jsons",
//    "dist": "examples/dist"
//  }

// examples/jsons is contain below json files
//
// examples/jsons/test.json
// examples/jsons/nests/test-child.json
// examples/jsons/nests/child/json-placeholder.json
// examples/jsons/nests/child/array.json

fn main() {
    let config = FileToFileConfig::from_file("config.json").unwrap();
    let generator= RustTypeDescriptionGeneratorBuilder::new()
        // add comment at Declare part
        .declare_part_all_comment("this is auto generate")
        // add derive with serde
        .declare_part_set_all_derive_with_serde(vec!["Debug", "Clone"])
        // all type is declare pub
        .declare_part_pub_all()
        // all property is declare option
        .property_part_all_optional()
        // all property is declare pub
        .property_part_pub_all()
        .build();

    // to dist dirs
    json_to_rust(config, generator);
}


// after run above code, examples is contain below json files
//
// examples/dist.rs
pub mod test;
pub mod nests;
// examples/dist/test.rs
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
// this is auto generate
pub struct Test {
    pub id: Option<usize>,
    pub name: Option<String>,
    pub obj: Option<TestObj>,
}
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
// this is auto generate
pub struct TestObj {
    pub age: Option<usize>,
    pub from: Option<String>,
    pub now: Option<String>,
}
// examples/dist/nests.rs
pub mod test_child;
pub mod child;

// examples/dist/nests/child/test_child.rs
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
// this is auto generate
pub struct TestChild {
    pub child: Option<Vec<TestChildChild>>,
    pub id: Option<usize>,
}
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
// this is auto generate
pub struct TestChildChild {
    pub hello: Option<String>,
}

// examples/dist/nests/child/child.rs
pub mod json_placeholder;
pub mod array;

// examples/dist/nests/child/json-placeholder.rs
// this is auto generate
pub type JsonPlaceholderArray = Vec<JsonPlaceholder>;
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
// this is auto generate
pub struct JsonPlaceholder {
    pub body: Option<String>,
    pub id: Option<usize>,
    pub title: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<usize>,
}
// examples/dist/nests/child/array.rs
// this is auto generate
pub type ArrayArray = Vec<Array>;
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
// this is auto generate
pub struct Array {
    pub arr: Option<Vec<ArrayArr>>,
    pub greet: Option<String>,
    pub id: Option<usize>,
}

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
// this is auto generate
pub struct ArrayArr {
    pub data: Option<ArrayArrData>,
}

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
// this is auto generate
pub struct ArrayArrData {
    pub id: Option<usize>,
}


```
