use std::net::SocketAddr;

use anyhow::Context;
use rust::database::init_database;
use rust::routes::create_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL env var not set")?;

    let pool = init_database(&database_url).await?;

    let app = create_router(pool);

    let address = SocketAddr::from(([0, 0, 0, 0], 4000));

    axum_server::bind(address)
        .serve(app.into_make_service())
        .await
        .context("Server failed to start")?;

    Ok(())
}
