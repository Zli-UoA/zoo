use async_graphql::SimpleObject;

#[derive(SimpleObject, Debug, PartialEq, Eq)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub archived: bool,
    pub private: bool,
}
