use async_graphql::SimpleObject;

#[derive(SimpleObject, Debug)]
pub struct TestObj {
    pub id: String,
    pub name: String,
    pub num: usize,
}
