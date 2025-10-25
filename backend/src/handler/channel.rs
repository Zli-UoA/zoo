use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;

use crate::context::Context;
use crate::models;
use crate::usecase;

#[derive(Default)]
pub struct ChannelListQuery;
#[derive(Default)]
pub struct ChannelOwnerQuery;
#[derive(Default)]
pub struct ChannelMembersQuery;

#[Object]
impl ChannelOwnerQuery {
    async fn channel_owner<'ctx>(
        &self,
        ctx: &async_graphql::Context<'ctx>,
        id: String,
    ) -> Result<models::user::User> {
        let ctx = ctx.data_unchecked::<Context>();

        match usecase::channel::get_channel_owner_by_channel_id(ctx, id.as_str()).await {
            Ok(user) => Ok(user),
            Err(_) => Err(Error::new("Internal server error")),
        }
    }
}

#[Object]
impl ChannelListQuery {
    async fn channels<'ctx>(
        &self,
        ctx: &async_graphql::Context<'ctx>,
    ) -> Result<Vec<models::channel::Channel>> {
        let ctx = ctx.data_unchecked::<Context>();

        match usecase::channel::get_all_channel(ctx).await {
            Ok(channels) => Ok(channels),
            Err(_) => Err(Error::new("Internal server error")),
        }
    }
}

#[Object]
impl ChannelMembersQuery {
    async fn channel_users<'ctx>(
        &self,
        ctx: &async_graphql::Context<'ctx>,
        id: String,
    ) -> Result<Vec<models::user::User>> {
        let ctx = ctx.data_unchecked::<Context>();

        match usecase::channel::get_channel_users_by_channel_id(ctx, id.as_str()).await {
            Ok(channels) => Ok(channels),
            Err(_) => Err(Error::new("Internal server error")),
        }
    }
}
