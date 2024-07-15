use zoo_backend::server::server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    server::run_server().await?;

    Ok(())
}
