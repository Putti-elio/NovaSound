use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use anyhow::Context;
use rust::database::init_database;
use rust::routes::create_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let database = init_database().context("Failed to initialize database")?;

    let shared_database = Arc::new(Mutex::new(database));

    let app = create_router(shared_database);

    let address = SocketAddr::from(([0, 0, 0, 0], 4000));

    axum_server::bind(address)
        .serve(app.into_make_service())
        .await
        .context("Server failed to start")?;

    Ok(())
}
