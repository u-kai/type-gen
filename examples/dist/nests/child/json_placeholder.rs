use serde::{Deserialize,Serialize};
// this is auto make type
pub type JsonPlaceholderArray = Vec<JsonPlaceholder>;
// this is auto make type
#[allow(unused)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct JsonPlaceholder {
    // this is auto make property
    #[allow(unused)]
    pub body: Option<String>,
    // this is auto make property
    #[allow(unused)]
    pub id: Option<usize>,
    // this is auto make property
    #[allow(unused)]
    pub title: Option<String>,
    // this is auto make property
    #[allow(unused)]
    #[serde(rename = "userId")]
    pub user_id: Option<usize>,
}
