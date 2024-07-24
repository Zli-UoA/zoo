use crate::handler::mutation::Mutation;
use crate::handler::query::Query;
use async_graphql::{EmptySubscription, Schema, SchemaBuilder};

pub fn create_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    let schema_builder = Schema::build(Query::default(), Mutation::default(), EmptySubscription);

    schema_builder
}
