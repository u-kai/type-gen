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
