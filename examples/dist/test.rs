use serde::{Deserialize,Serialize};
// this is auto make type
#[allow(unused)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Test {
    // this is auto make property
    #[allow(unused)]
    pub id: Option<usize>,
    // this is auto make property
    #[allow(unused)]
    pub name: Option<String>,
    // this is auto make property
    #[allow(unused)]
    pub obj: Option<TestObj>,
}
// this is auto make type
#[allow(unused)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct TestObj {
    // this is auto make property
    #[allow(unused)]
    pub age: Option<usize>,
    // this is auto make property
    #[allow(unused)]
    pub from: Option<String>,
    // this is auto make property
    #[allow(unused)]
    pub now: Option<String>,
}
