use std::net::SocketAddr;

use anyhow::Context;
use axum::{
    Router,
    http::{HeaderValue, Method},
};
use rust::database::init_database;
use rust::services::connect::create_connect_router;
use rust::state::AppState;
use tower_http::cors::CorsLayer;

fn cors_layer() -> anyhow::Result<CorsLayer> {
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    let layer = CorsLayer::new().allow_methods([Method::GET]);

    if app_env == "dev" {
        Ok(layer.allow_origin([
            HeaderValue::from_static("http://localhost:5173"),
            HeaderValue::from_static("tauri://localhost"),
        ]))
    } else {
        let origin = std::env::var("WEB_ORIGIN")
            .context("WEB_ORIGIN env var must be set outside development")?
            .parse::<HeaderValue>()
            .context("WEB_ORIGIN must be a valid HTTP origin")?;

        Ok(layer.allow_origin(origin))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL env var not set")?;

    let pool = init_database(&database_url).await?;

    let app_state = AppState::new(pool);
    let connect_router = create_connect_router(app_state.clone());
    let app = Router::new()
        .nest("/web", rust::web::router())
        .fallback_service(connect_router.into_axum_service())
        .layer(cors_layer()?)
        .with_state(app_state);

    let address = SocketAddr::from(([0, 0, 0, 0], 4000));

    axum_server::bind(address)
        .serve(app.into_make_service())
        .await
        .context("Server failed to start")?;

    Ok(())
}
