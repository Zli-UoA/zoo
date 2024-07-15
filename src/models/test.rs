use async_graphql::{InputObject, SimpleObject};

#[derive(SimpleObject, Debug)]
pub struct TestObj {
    pub id: String,
    pub name: String,
    pub num: usize,
}

#[derive(InputObject)]
pub struct CreateTestObjInput {
    pub name: String,
    pub num: usize,
}
