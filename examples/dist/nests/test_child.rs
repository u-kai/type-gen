use serde::{Deserialize,Serialize};
// this is auto make type
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct TestChild {
    // this is auto make property
    #[allow(unuse)]
    pub child: Option<Vec<TestChildChild>>,
    // this is auto make property
    #[allow(unuse)]
    pub id: Option<usize>,
}
// this is auto make type
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct TestChildChild {
    // this is auto make property
    #[allow(unuse)]
    pub hello: Option<String>,
}
