use serde::{Deserialize,Serialize};
// this is auto make type
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Test {
    // this is auto make property
    #[allow(unuse)]
    pub id: Option<usize>,
    // this is auto make property
    #[allow(unuse)]
    pub name: Option<String>,
    // this is auto make property
    #[allow(unuse)]
    pub obj: Option<TestObj>,
}
// this is auto make type
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct TestObj {
    // this is auto make property
    #[allow(unuse)]
    pub age: Option<usize>,
    // this is auto make property
    #[allow(unuse)]
    pub from: Option<String>,
    // this is auto make property
    #[allow(unuse)]
    pub now: Option<String>,
}
