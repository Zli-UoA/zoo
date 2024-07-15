use crate::handler::mutation::Mutation;
use crate::handler::query::Query;
use async_graphql::{EmptySubscription, Schema};

pub fn create_schema() -> Schema<Query, Mutation, EmptySubscription> {
    let schema = Schema::new(Query::default(), Mutation::default(), EmptySubscription);
    schema
}
