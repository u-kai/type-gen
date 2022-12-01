use serde::{Deserialize,Serialize};
// this is auto make type
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub type JsonPlaceholderArray = Vec<JsonPlaceholder>;
// this is auto make type
#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct JsonPlaceholder {
    // this is auto make property
    #[allow(unuse)]
    pub body: Option<String>,
    // this is auto make property
    #[allow(unuse)]
    pub id: Option<usize>,
    // this is auto make property
    #[allow(unuse)]
    pub title: Option<String>,
    // this is auto make property
    #[allow(unuse)]
    #[serde(rename = "userId")]
    pub user_id: Option<usize>,
}
