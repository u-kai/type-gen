use serde::{Deserialize,Serialize};
// this is auto make type
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub type ArrayArray = Vec<Array>;
// this is auto make type
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Array {
    // this is auto make property
    #[allow(unuse)]
    pub arr: Option<Vec<ArrayArr>>,
    // this is auto make property
    #[allow(unuse)]
    pub greet: Option<String>,
    // this is auto make property
    #[allow(unuse)]
    pub id: Option<usize>,
}

// this is auto make type
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct ArrayArr {
    // this is auto make property
    #[allow(unuse)]
    pub data: Option<ArrayArrData>,
}

// this is auto make type
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct ArrayArrData {
    // this is auto make property
    #[allow(unuse)]
    pub id: Option<usize>,
}
