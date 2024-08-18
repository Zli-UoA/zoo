use crate::{context::Context, models::user::User};

pub async fn get_user_by_id(_ctx: &Context, _id: &str) -> Result<User, ()> {
    todo!();
}

#[cfg(test)]
pub mod test {
    use sea_orm::prelude::*;
    use sea_orm::DatabaseConnection;
    use sea_orm::MockDatabase;

    use crate::context::Context;
    use crate::generate::entities::user;
    use crate::models::user::User;
    use crate::usecase::user::get_user_by_id;

    #[tokio::test]
    async fn 指定IDのユーザが取得される() {
        // Arrange
        let id = "4e36eb58-49a5-43aa-935f-5a5cccb77a90";

        let db: DatabaseConnection = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![user::Model {
                id: id.to_string(),
                user_name: "aiueo".to_string(),
                display_name: "あいうえお".to_string(),
                created_at: DateTime::parse_from_str("2024-08-08 00:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                updated_at: DateTime::parse_from_str("2024-08-08 00:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
            }]])
            .into_connection();

        let ctx = Context {
            env: "".to_string(),
            db,
        };

        // Action
        let result = get_user_by_id(&ctx, id).await;

        // Assert
        assert_eq!(
            result,
            Ok(User {
                id: id.to_string(),
                name: "aiueo".to_string(),
                display_name: "あいうえお".to_string(),
            })
        );
    }

    #[tokio::test]
    async fn 指定IDのユーザが存在しない場合失敗() {
        // Arrange
        let id = "4e36eb58-49a5-43aa-935f-5a5cccb77a90";

        let db: DatabaseConnection = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([vec![user::Model {
                id: id.to_string(),
                user_name: "aiueo".to_string(),
                display_name: "あいうえお".to_string(),
                created_at: DateTime::parse_from_str("2024-08-08 00:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                updated_at: DateTime::parse_from_str("2024-08-08 00:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
            }]])
            .into_connection();

        let ctx = Context {
            env: "".to_string(),
            db,
        };

        // Action
        let result = get_user_by_id(&ctx, "存在しないID").await;

        // Assert
        assert_eq!(result, Err(()));
    }
}
