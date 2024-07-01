use axum::{
    routing::{get, post},
    Router,
    response::Response
};

pub fn index() -> String {
    format!("Hello this is our TikTok Techjam API website")
}
