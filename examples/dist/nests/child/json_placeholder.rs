#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
// this is auto generate
pub type JsonPlaceholderArray = Vec<JsonPlaceholder>;
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
// this is auto generate
pub struct JsonPlaceholder {
    pub body: Option<String>,
    pub id: Option<usize>,
    pub title: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<usize>,
}
