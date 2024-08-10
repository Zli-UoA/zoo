use std::time::Duration;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tokio::net::TcpListener;

use super::schema::create_schema;
use crate::context::Context;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn run_server() -> Result<(), std::io::Error> {
    let schema = create_schema()
        .data(Context {
            env: "harukun".to_string(),
            db: connect_db("hoge").await.expect("Cannot connect DB"),
        })
        .finish();

    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    axum::serve(TcpListener::bind("0.0.0.0:8000").await.unwrap(), app).await?;

    println!("GraphiQL IDE: http://localhost:8000");

    Ok(())
}

async fn connect_db(url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut connect_opt = ConnectOptions::new(url);
    connect_opt
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .set_schema_search_path("my_schema");

    Database::connect(connect_opt).await
}
