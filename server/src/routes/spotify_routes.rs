use axum::{
    extract::{State, Json},
    routing::post,
    Router,
    http::status,
    response::{Response, IntoResponse},
    body::Body
};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::AppState;
use crate::spotify;

#[derive(Deserialize, Serialize)]
struct SpotifyInput {
    user_id: String
}

pub fn spotify_routes(appstate: AppState) -> Router {
    Router::new()
        .route("/api/v1/spotify/login",post(post_spotify_login))
        .with_state(appstate)
}

async fn post_spotify_login(State(appstate): State<AppState>, Json(payload): Json<SpotifyInput>) -> Response {

    let user_id = match ObjectId::parse_str(&payload.user_id) {
        Ok(u) => u,
        Err(_) => {
        return Response::builder()
            .status(status::StatusCode::BAD_REQUEST)
            .header("Content-Type","plain/text")
            .body(Body::from("Not a valid user_id"))
            .unwrap()
        },
    };
    let token = match spotify::auth::spotify_auth(appstate.user_collection.clone(), user_id).await {
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
