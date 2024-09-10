use crate::handler::test_handler::TestQuery;
use async_graphql::MergedObject;

use crate::handler::user::UserQuery;

#[derive(MergedObject, Default)]
pub struct Query(TestQuery, UserQuery);
