#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Test {
    #[allow(unuse)]
    pub id: Option<usize>,
    #[allow(unuse)]
    pub name: Option<String>,
    #[allow(unuse)]
    pub obj: Option<TestObj>,
}

#[allow(unuse)]
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct TestObj {
    #[allow(unuse)]
    pub age: Option<usize>,
    #[allow(unuse)]
    pub from: Option<String>,
    #[allow(unuse)]
    pub now: Option<String>,
}