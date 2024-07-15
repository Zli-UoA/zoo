use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;

use super::schema::create_schema;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn run_server() -> Result<(), std::io::Error> {
    let schema = create_schema();
    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8000");

    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app).await?;

    Ok(())
}
