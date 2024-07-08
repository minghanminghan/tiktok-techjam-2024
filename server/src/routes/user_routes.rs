use axum::{
    extract::{Json, Extension},
    routing::post,
    Router,
    response::Response,
    http::StatusCode,
    body::Body
};
use serde_json::json;
use serde::Deserialize;
use tokio_postgres::Client;
use std::sync::Arc;

use crate::User;
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

pub fn user_routes() -> Router {
    Router::new()
        .route("/api/v1/login",post(post_login))
        .route("/api/v1/register",post(post_register))
}

async fn post_login(Extension(client): Extension<Arc<Client>>, Json(payload): Json<LoginInput>) -> Response {
    match login::login(&client, &payload.identifier, &payload.password).await {
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

async fn post_register(Extension(client): Extension<Arc<Client>>, Json(payload): Json<RegistrationInput>) -> Response {
    let user: User = match register::register(&client, &payload.username, &payload.email, &payload.password).await {
        Ok(u) => u,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(Body::from(err.to_string()))
                .unwrap();
        },
    };

    match login::login(&client, &user.username, &user.password).await {
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
