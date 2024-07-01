use axum::{
    extract::{Json, State},
    routing::post,
    Router,
    response::Response,
    http::StatusCode,
    body::Body

};
use serde_json::json;
use serde::Deserialize;

use crate::User;
use crate::AppState;
use crate::auth::login;
use crate::auth::register;

#[derive(Debug, Deserialize)]
struct LoginInput {
    identifier: String,
    password: String
}

#[derive(Debug, Deserialize)]
struct RegistrationInput {
    email: String,
    username: String,
    password: String
}

pub fn user_routes(appstate: AppState) -> Router {
    Router::new()
        .route("/api/v1/login",post(post_login))
        .route("/api/v1/register",post(post_register))
        .with_state(appstate)
}

async fn post_login(State(appstate): State<AppState>, Json(payload): Json<LoginInput>) -> Response {
    match login::login(appstate.user_collection.clone(), &payload.identifier, &payload.password).await {
        Ok(token) => {
            let response_body: serde_json::Value = json!({
                "token": token
            });

            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(response_body.to_string()))
                .unwrap()
        },
        Err(err) => {
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(Body::from(err.to_string()))
                .unwrap()
        },
    }
}

async fn post_register(State(appstate): State<AppState>, Json(payload): Json<RegistrationInput>) -> Response {
    let user: User;
    match register::register(appstate.user_collection.clone(), &payload.username, &payload.email, &payload.password).await {
        Ok(v) => {
            user = v
        },
        Err(err) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(Body::from(err.to_string()))
                .unwrap();
        },
    };

    match login::login(appstate.user_collection.clone(), &user.username, &user.password).await {
        Ok(token) => {
            let response_body: serde_json::Value = json!({
                "token": token
            });

            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(response_body.to_string()))
                .unwrap()
        },
        Err(err) => {
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(Body::from(err.to_string()))
                .unwrap()
        },
    }
}
