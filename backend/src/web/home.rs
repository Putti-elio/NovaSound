use askama::Template;
use axum::{extract::State, http::StatusCode, response::Html};

use crate::{models::artist_model::Artist, services::artist_service, state::AppState};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

#[derive(Template)]
#[template(path = "fragments/artists.html")]
struct ArtistsTemplate {
    artists: Vec<Artist>,
}

#[derive(Template)]
#[template(path = "fragments/backend-status.html")]
struct BackendStatusTemplate;

pub async fn home() -> Result<Html<String>, (StatusCode, Html<String>)> {
    HomeTemplate.render().map(Html).map_err(template_error)
}

pub async fn artists(
    State(state): State<AppState>,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    let artists = artist_service::get_all_artists(&state.db_pool)
        .await
        .map_err(|_| backend_error())?;

    ArtistsTemplate { artists }
        .render()
        .map(Html)
        .map_err(template_error)
}

pub async fn backend_status() -> Result<Html<String>, (StatusCode, Html<String>)> {
    BackendStatusTemplate
        .render()
        .map(Html)
        .map_err(template_error)
}

fn template_error(error: askama::Error) -> (StatusCode, Html<String>) {
    log::error!("Unable to render web template: {error}");
    backend_error()
}

fn backend_error() -> (StatusCode, Html<String>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Html("<p class=\"status-message\">Le catalogue est indisponible. Verifiez que le backend Axum est demarre.</p>".to_string()),
    )
}
