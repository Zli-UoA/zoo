use crate::test_handler::TestMutation;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Mutation(TestMutation);
