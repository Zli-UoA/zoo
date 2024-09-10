use crate::context::Context;
use crate::models::channel::Channel;

pub async fn get_all_channel(_ctx: &Context) -> Result<Vec<Channel>, ()> {
    todo!("generate::entities::channel::Model to models::channel::Channel")
}

// User型を返すように要修正
pub async fn get_channel_owner_by_channel_id(
    _ctx: &Context,
    _channel_id: &str,
) -> Result<String, ()> {
    todo!()
}

// User型を返すように要修正
pub async fn get_channel_users_by_channel_id(
    _ctx: &Context,
    _channel_id: &str,
) -> Result<Option<Vec<String>>, ()> {
    todo!()
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
            .append_query_results([vec![(
                channel::Model {
                    id: "0".to_string(),
                    channel_name: "hoge".to_string(),
                    description: Some("huga".to_string()),
                    is_private: false,
                    created_user_id: "aaa".to_string(),
                    created_at: DateTime::parse_from_str(
                        "2024-08-08 00:00:00",
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                    updated_at: None,
                    archive_at: None,
                    deleted_at: None,
                },
                user::Model {
                    id: "aaa".to_string(),
                    user_name: "haru".to_string(),
                    display_name: "haru".to_string(),
                    created_at: DateTime::parse_from_str(
                        "2024-08-08 00:00:00",
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                    updated_at: DateTime::parse_from_str(
                        "2024-08-08 00:00:00",
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                },
            )]])
            .into_connection();

        let context = Context {
            env: "harukun".to_string(),
            db,
        };

        // Action
        let result = get_channel_owner_by_channel_id(&context, "aaa")
            .await
            .unwrap();
        // Assert
        assert_eq!(result, "aaa".to_string())
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
