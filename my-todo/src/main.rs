use std::env;
use std::net::SocketAddr;

use anyhow::ensure;

use my_todo::app::create_app;
use my_todo::repositories::in_memory::TodoRepositoryInMemory;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ensure!(env::var("RUST_LOG").is_ok());
    tracing_subscriber::fmt::init();

    let repository = TodoRepositoryInMemory::new();
    let app = create_app(repository);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
