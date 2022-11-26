#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Child {
    #[allow(unuse)]
    pub child: Option<Vec<ChildChild>>,
    #[allow(unuse)]
    pub id: Option<usize>,
}

#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct ChildChild {
    #[allow(unuse)]
    pub hello: Option<String>,
}