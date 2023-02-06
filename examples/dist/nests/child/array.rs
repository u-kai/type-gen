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
