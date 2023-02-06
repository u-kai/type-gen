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
