use async_graphql::SimpleObject;

#[derive(SimpleObject, Debug, PartialEq, Eq)]
pub struct Channel {
    pub id: String,
    pub name: String,
    // Animal型を実装したら書き換える
    pub owner: String,
    pub description: Option<String>,
    pub created_at: String,
    pub archived: bool,
    pub private: bool,
    // Animal型を実装したら書き換える
    pub animals: Vec<String>,
    // message型を実装したら書き換える
    pub messages: Vec<String>,
}
