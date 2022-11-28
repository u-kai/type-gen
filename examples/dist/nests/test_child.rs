use serde_json::Value;
use serde::{Deserialize,Serialize};
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct TestChild {
    #[allow(unuse)]
    pub child: Option<Vec<TestChildChild>>,
    #[allow(unuse)]
    pub id: Option<usize>,
}
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct TestChildChild {
    #[allow(unuse)]
    pub hello: Option<String>,
}
