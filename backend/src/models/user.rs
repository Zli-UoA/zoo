use async_graphql::SimpleObject;

#[derive(SimpleObject, Debug, Eq, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub display_name: String,
}
