use axum::{
    extract::{Extension, Json},
    routing::post,
    Router,
    http::status,
    response::{Response, IntoResponse},
    body::Body
};
use axum_extra::extract::cookie::{CookieJar, Cookie};
use tokio_postgres::Client;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::auth::login::verify_jwt;

use crate::spotify;

#[derive(Deserialize, Serialize)]
struct SpotifyInput {
    user_id: String
}

pub fn spotify_routes() -> Router {
    Router::new()
        .route("/api/v1/spotify/login",post(post_spotify_login))
        .route("/api/v1/spotify/like",post(post_spotify_swipe))
        .route("/api/v1/spotify/fetch",post(post_spotify_fetch))
}

async fn post_spotify_login(Extension(client): Extension<Arc<Client>>, jar: CookieJar) -> Response {

    let jwt = match jar.get("token") {
        Some(t) => t,
        None => return Response::builder()
            .status(status::StatusCode::UNAUTHORIZED)
            .body(Body::from("No cookie"))
            .unwrap()
    };
    let claims = match verify_jwt(&jwt.to_string()) {
        Ok(c) => c,
        Err(_) => return Response::builder()
            .status(status::StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized cookie"))
            .unwrap()
    };
    let user_id: i32 = match claims.sub.parse::<i32>() {
        Ok(i) => i,
        Err(_) => return Response::builder()
            .status(status::StatusCode::UNAUTHORIZED)
            .body(Body::from("Invalid cookie sub"))
            .unwrap()
    };
    
    let token = match spotify::auth::spotify_auth(client, user_id).await {
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

async fn post_spotify_swipe(Extension(client): Extension<Arc<Client>>, Json(payload): Json<SpotifyInput>, jar: CookieJar) -> Response {

    let jwt = match jar.get("token") {
        Some(t) => t,
        None => return Response::builder()
            .status(status::StatusCode::UNAUTHORIZED)
            .body(Body::from("No cookie"))
            .unwrap()
    };
    let claims = match verify_jwt(jwt) {
        Ok(c) => c,
        Err(_) => return Response::builder()
            .status(status::StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized cookie"))
            .unwrap()
    };
    let user_id = claims.sub.parse();

    match spotify::swipe::like_song(payload.song).await {
        Ok(_) => "Sucessfully swiped".into_response(),
        Err(err) => "".into_response()
    }
}

async fn post_spotify_fetch(Extension(client): Extension<Arc<Client>>, Json(payload): Json<SpotifyInput>, jar: CookieJar) -> Response {

    let jwt = match jar.get("token") {
        Some(t) => t,
        None => return Response::builder()
            .status(status::StatusCode::UNAUTHORIZED)
            .body(Body::from("No cookie"))
            .unwrap()
    };
    let claims = match verify_jwt(jwt) {
        Ok(c) => c,
        Err(_) => return Response::builder()
            .status(status::StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized cookie"))
            .unwrap()
    };
    let user_id = claims.sub.parse();

    let token = match spotify::auth::spotify_auth(client, user_id).await {
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
