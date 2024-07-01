use axum::{
    routing::{get, post},
    Router,
    http::status,
    response::{Response, IntoResponse},
    body::Body
};
use mongodb::{Collection};

use crate::AppState;
use crate::Song;

pub fn spotify_routes(appstate: AppState) -> Router {
    Router::new()
        .route("/api/v1/spotify/login",post(post_spotify_login))
        .with_state(appstate)
}

async fn post_spotify_login() -> Response {
    Response::builder()
        .status(status::StatusCode::NOT_IMPLEMENTED)
        .header("Content-Type","plain/text")
        .body(Body::from("Shit not made yet".to_string()))
        .unwrap()
}
