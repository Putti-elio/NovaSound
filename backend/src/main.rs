use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use rust::database::init_database;
use rust::services::connect::create_connect_router;
use rust::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL env var not set")?;

    let pool = init_database(&database_url).await?;

    let app_state = AppState::new(pool);
    let connect_router = create_connect_router(app_state);
    let app = Router::new().fallback_service(connect_router.into_axum_service());

    let address = SocketAddr::from(([0, 0, 0, 0], 4000));

    axum_server::bind(address)
        .serve(app.into_make_service())
        .await
        .context("Server failed to start")?;

    Ok(())
}
