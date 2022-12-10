use serde::{Deserialize,Serialize};
// this is auto make type
pub type ArrayArray = Vec<Array>;
// this is auto make type
#[allow(unused)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Array {
    // this is auto make property
    #[allow(unused)]
    pub arr: Option<Vec<ArrayArr>>,
    // this is auto make property
    #[allow(unused)]
    pub greet: Option<String>,
    // this is auto make property
    #[allow(unused)]
    pub id: Option<usize>,
}

// this is auto make type
#[allow(unused)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct ArrayArr {
    // this is auto make property
    #[allow(unused)]
    pub data: Option<ArrayArrData>,
}

// this is auto make type
#[allow(unused)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct ArrayArrData {
    // this is auto make property
    #[allow(unused)]
    pub id: Option<usize>,
}
