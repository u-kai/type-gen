use serde::{Deserialize,Serialize};
// this is auto make type
#[allow(unused)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct TestChild {
    // this is auto make property
    #[allow(unused)]
    pub child: Option<Vec<TestChildChild>>,
    // this is auto make property
    #[allow(unused)]
    pub id: Option<usize>,
}
// this is auto make type
#[allow(unused)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct TestChildChild {
    // this is auto make property
    #[allow(unused)]
    pub hello: Option<String>,
}
