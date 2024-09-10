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
    use super::{
        get_all_channel, get_channel_owner_by_channel_id, get_channel_users_by_channel_id,
        get_messages_by_channel_id,
    };
    use crate::context::Context;
    use crate::generate::entities::channel;
    use crate::generate::entities::channel_user;
    use crate::generate::entities::message;
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
                //users: vec!["eraser".to_string(), "yuorei".to_string()],
                //messages: vec!["welcome".to_string(), "zoo".to_string()]
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

    // append_query_resultsにVectorを含むタプルを渡すための拡張
    use sea_orm::{
        EntityTrait, IdenStatic, IntoMockRow, Iterable, MockRow, ModelTrait, SelectA, SelectB,
    };
    use std::collections::BTreeMap;

    struct IntoMockVecCulumn<M, N>(M, Vec<N>)
    where
        M: ModelTrait,
        N: ModelTrait;

    impl<M, N> IntoMockRow for IntoMockVecCulumn<M, N>
    where
        M: ModelTrait,
        N: ModelTrait,
    {
        fn into_mock_row(self) -> MockRow {
            let mut mapped_join = BTreeMap::new();

            for column in <<M as ModelTrait>::Entity as EntityTrait>::Column::iter() {
                mapped_join.insert(
                    format!("{}{}", SelectA.as_str(), column.as_str()),
                    self.0.get(column),
                );
            }
            for column in <<N as ModelTrait>::Entity as EntityTrait>::Column::iter() {
                    for c in self.1.iter() {
                        mapped_join.insert(
                            format!("{}{}", SelectB.as_str(), column.as_str()),
                            c.get(column)
                        );
                    }
            }

            mapped_join.into_mock_row()
        }
    }

    #[tokio::test]
    async fn チャンネルに参加しているユーザーの一覧を取得する() {
        // Arrange
        let db: DatabaseConnection = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            // first query result
            .append_query_results([vec![IntoMockVecCulumn(
                channel::Model {
                    id: "0".to_string(),
                    channel_name: "hoge".to_string(),
                    description: Some("huga".to_string()),
                    is_private: false,
                    created_user_id: "aaa".to_string(),
                    created_at: DateTime::parse_from_str(
                        "2024-08-08 00:00:00",
                        "%Y-%m-%d %H:%M:%S",
                    ).unwrap(),
                    updated_at: None,
                    archive_at: None,
                    deleted_at: None,
                },
                vec![
                channel_user::Model {
                    joined_at: DateTime::parse_from_str("2024-08-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                    user_id: "aaa".to_string(),
                    channel_id: "0".to_string(),
                },
                channel_user::Model {
                    joined_at: DateTime::parse_from_str("2024-08-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                    user_id: "bbb".to_string(),
                    channel_id: "0".to_string(),
                }
                ])]])
            // second query result
            .append_query_results([vec![(
                    user::Model {
                        id: "aaa".to_string(),
                        user_name: "haru".to_string(),
                        display_name: "haru".to_string(),
                        created_at: DateTime::parse_from_str(
                            "2024-08-08 00:00:00",
                            "%Y-%m-%d %H:%M:%S",
                        ).unwrap(),
                        // Noneになる予定？
                        updated_at: DateTime::parse_from_str(
                            "2024-08-08 00:00:00",
                            "%Y-%m-%d %H:%M:%S",
                        ).unwrap(),
                    },
                    user::Model {
                        id: "bbb".to_string(),
                        user_name: "eraser".to_string(),
                        display_name: "eraser".to_string(),
                        created_at: DateTime::parse_from_str(
                            "2024-08-08 00:00:00",
                            "%Y-%m-%d %H:%M:%S",
                        ).unwrap(),
                        // Noneになる予定？
                        updated_at: DateTime::parse_from_str(
                            "2024-08-08 00:00:00",
                            "%Y-%m-%d %H:%M:%S",
                        ).unwrap(),
                    }
            )]])
            .into_connection();

        let context = Context {
            env: "harukun".to_string(),
            db,
        };

        // Action
        let result = get_channel_users_by_channel_id(&context, "aaa")
            .await
            .unwrap()
            .unwrap();
        // Assert
        assert_eq!(result[0], "aaa".to_string()) // message型を実装したらこのテストも修正する
    }

    #[tokio::test]
    async fn 全てのメッセージを取得する() {
        // Arrange
        let db: DatabaseConnection = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![IntoMockVecCulumn(
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
                vec![
                message::Model {
                    id: "aaa".to_string(),
                    user_id: "harukun".to_string(),
                    channel_id: "0".to_string(),
                    content: "test".to_string(),
                    created_at: DateTime::parse_from_str(
                        "2024-08-08 00:00:00",
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                    updated_at: None,
                    deleted_at: None,
                },
                message::Model {
                    id: "bbb".to_string(),
                    user_id: "eraser".to_string(),
                    channel_id: "0".to_string(),
                    content: "test2".to_string(),
                    created_at: DateTime::parse_from_str(
                        "2024-08-08 00:00:00",
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                    updated_at: None,
                    deleted_at: None,
                },
                ]
            )]])
            .into_connection();

        let context = Context {
            env: "harukun".to_string(),
            db,
        };

        // Action
        let result = get_messages_by_channel_id(&context, "aaa")
            .await
            .unwrap()
            .unwrap();
        // Assert
        assert_eq!(result[0], "aaa".to_string()) // message型を実装したらこのテストも修正する
    }
}
