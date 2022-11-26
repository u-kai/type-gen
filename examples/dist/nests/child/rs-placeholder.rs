#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct JsonPlaceholder {
    #[allow(unuse)]
    pub body: Option<String>,
    #[allow(unuse)]
    pub id: Option<usize>,
    #[allow(unuse)]
    pub title: Option<String>,
    #[allow(unuse)]
    #[serde(rename = "userId")]
    pub user_id: Option<usize>,
}