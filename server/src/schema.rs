use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
use handler::mutation::Mutation;
use handler::query::Query;

pub fn create_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    let schema_builder = Schema::build(Query::default(), Mutation::default(), EmptySubscription);

    schema_builder
}
