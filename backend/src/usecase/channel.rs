use async_graphql::{Error, Result};
use sea_orm::{EntityTrait, ModelTrait};

use crate::context::Context;
use crate::generate::entities::channel_user::Entity as Member;
use crate::generate::entities::user::{self, Entity as Owner};
use crate::models::{channel::Channel, user::User};

use crate::generate::entities::channel;

pub async fn get_all_channel(ctx: &Context) -> Result<Vec<Channel>, String> {
    let Ok(channel) = channel::Entity::find().all(&ctx.db).await else {
        return Err("DB error".to_string());
    };

    Ok(channel
        .into_iter()
        .map(|channel| Channel {
            id: channel.id,
            name: channel.channel_name,
            description: channel.description,
            created_at: channel.created_at.to_string(),
            archived: channel.archive_at.is_some(),
            private: channel.is_private,
        })
        .collect())
}

pub async fn get_channel_owner_by_channel_id(
    ctx: &Context,
    channel_id: &str,
) -> Result<User, String> {
    let Ok(Some(channel)) = channel::Entity::find_by_id(channel_id).one(&ctx.db).await else {
        return Err("DB error. channel could not be found".to_string());
    };

    let Ok(owner) = channel.find_related(Owner).one(&ctx.db).await else {
        return Err("DB error. channel owner could not be found".to_string());
    };

    if let Some(owner) = owner {
        Ok(User {
            id: owner.id,
            name: owner.user_name,
            display_name: owner.display_name,
        })
    } else {
        Err("owner could not be found.".to_string())
    }
}

pub async fn get_channel_users_by_channel_id(
    ctx: &Context,
    channel_id: &str,
) -> Result<Vec<User>, String> {
    let Ok(Some(channel)) = channel::Entity::find_by_id(channel_id).one(&ctx.db).await else {
        return Err("DB error. channel could not be found".to_string());
    };

    let Ok(members) = channel.find_related(Member).all(&ctx.db).await else {
        return Err("DB error. channel members could not be found.".to_string());
    };

    let mut errors = Vec::new();
    let futures = members
        .into_iter()
        .map(async |member| get_user_by_channel_user(ctx, &member.user_id).await);
    let results = futures::future::join_all(futures).await;
    let users: Vec<User> = results
        .into_iter()
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .filter_map(|x| x)
        .collect();
    if !errors.is_empty() {
        return Err(format!("failed get user: {:?}", errors));
    } else {
        Ok(users)
    }
}

async fn get_user_by_channel_user(ctx: &Context, id: &str) -> Result<Option<User>, String> {
    let Ok(user) = user::Entity::find_by_id(id).one(&ctx.db).await else {
        return Err("DB error".to_string());
    };

    Ok(user.map(|user| User {
        id: user.id,
        name: user.user_name,
        display_name: user.display_name,
    }))
}

// Message型を返すように要修正
pub async fn get_messages_by_channel_id(
    _ctx: &Context,
    _channel_id: &str,
) -> Result<Option<Vec<String>>, ()> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::{get_all_channel, get_channel_owner_by_channel_id};
    use crate::context::Context;
    use crate::generate::entities::channel;
    use crate::generate::entities::user;
    use crate::models::channel::Channel;
    use crate::models::user::User;
    use sea_orm::prelude::*;
    use sea_orm::MockDatabase;

    #[tokio::test]
    async fn すべてのチャンネルを取得する() {
        // Arrange
        let db: DatabaseConnection = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![channel::Model {
                id: "0".to_string(),
                channel_name: "hoge".to_string(),
                description: Some("huga".to_string()),
                is_private: false,
                created_user_id: "aaa".to_string(),
                created_at: DateTime::parse_from_str("2024-08-08 00:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                updated_at: None,
                archive_at: None,
                deleted_at: None,
            }]])
            .into_connection();

        let context = Context {
            env: "harukun".to_string(),
            db,
        };

        // Action
        let result = get_all_channel(&context).await.unwrap();

        // Assert
        assert_eq!(
            result,
            vec![Channel {
                id: "0".to_string(),
                name: "hoge".to_string(),
                description: Some("huga".to_string()),
                created_at: "2024-08-08 00:00:00".to_string(),
                archived: false,
                private: false,
            }],
        )
    }

    #[tokio::test]
    async fn チャンネルオーナーを取得する() {
        // Arrange
        let db: DatabaseConnection = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![channel::Model {
                id: "0".to_string(),
                channel_name: "hoge".to_string(),
                description: Some("huga".to_string()),
                is_private: false,
                created_user_id: "aaa".to_string(),
                created_at: DateTime::parse_from_str("2024-08-08 00:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                updated_at: None,
                archive_at: None,
                deleted_at: None,
            }]])
            .append_query_results([vec![user::Model {
                id: "aaa".to_string(),
                user_name: "haru".to_string(),
                display_name: "haru".to_string(),
                created_at: DateTime::parse_from_str("2024-08-08 00:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                updated_at: Some(
                    DateTime::parse_from_str("2024-08-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                ),
            }]])
            .into_connection();

        let context = Context {
            env: "harukun".to_string(),
            db,
        };

        // Action
        let result = get_channel_owner_by_channel_id(&context, "0")
            .await
            .unwrap();
        // Assert
        assert_eq!(
            result,
            User {
                id: "aaa".to_string(),
                name: "haru".to_string(),
                display_name: "haru".to_string()
            }
        )
    }

    #[tokio::test]
    async fn チャンネルに参加しているユーザーの一覧を取得する() {
        // SeaORMのMockの仕様上find_with_related()のシミュレートが出来ないためテストが不可能
    }

    #[tokio::test]
    async fn 全てのメッセージを取得する() {
        // SeaORMのMockの仕様上find_with_related()のシミュレートが出来ないためテストが不可能
    }
}
