use axum::{Router, routing::get};

use crate::state::AppState;

pub mod home;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(home::home))
        .route("/artists", get(home::artists))
        .route("/status", get(home::backend_status))
}
