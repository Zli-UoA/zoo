use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;

use crate::context::Context;
use crate::models;
use crate::usecase;

#[derive(Default)]
pub struct UserQuery;
pub struct UserListQuery;

#[Object]
impl UserQuery {
    async fn user<'ctx>(
        &self,
        ctx: &async_graphql::Context<'ctx>,
        id: String,
    ) -> Result<models::user::User> {
        let ctx = ctx.data_unchecked::<Context>();

        match usecase::user::get_user_by_id(ctx, id.as_str()).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(Error::new("Not found")),
            Err(_) => Err(Error::new("Internal server error")),
        }
    }
}

#[Object]
impl UserListQuery {
    async fn users<'ctx>(
        &self,
        ctx: &async_graphql::Context<'ctx>,
    ) -> Result<Vec<models::user::User>> {
        let ctx = ctx.data_unchecked::<Context>();

        match usecase::user::get_all_users(ctx).await {
            Ok(users) => Ok(users),
            Err(_) => Err(Error::new("Internal server error")),
        }
    }
}