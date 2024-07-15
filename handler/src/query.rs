use crate::test_handler::TestQuery;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Query(TestQuery);
