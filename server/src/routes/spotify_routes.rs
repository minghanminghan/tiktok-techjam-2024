use axum::{
    extract::{Extension, Json},
    routing::post,
    Router,
    http::status,
    response::{Response, IntoResponse},
    body::Body
};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

use crate::spotify;

#[derive(Deserialize, Serialize)]
struct SpotifyInput {
    user_id: String
}

pub fn spotify_routes() -> Router {
    Router::new()
        .route("/api/v1/spotify/login",post(post_spotify_login))
}

async fn post_spotify_login(Extension(pool): Extension<PgPool>, Json(payload): Json<SpotifyInput>) -> Response {

    let user_id: i32 = match payload.user_id.parse() {
        Ok(u) => u,
        Err(_) => {
        return Response::builder()
            .status(status::StatusCode::BAD_REQUEST)
            .header("Content-Type","plain/text")
            .body(Body::from("Not a valid user_id"))
            .unwrap()
        },
    };
    let token = match spotify::auth::spotify_auth(&pool, user_id).await {
        Ok(t) => t,
        Err(err) => {
        return Response::builder()
            .status(status::StatusCode::BAD_REQUEST)
            .header("Content-Type","plain/text")
            .body(Body::from(err.to_string()))
            .unwrap()
        },
    };
    Json(token).into_response()
}
