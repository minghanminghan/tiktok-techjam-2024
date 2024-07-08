use axum::{
    routing::get,
    Router
};

use crate::routes::handle_get;
use crate::routes::{spotify_routes::spotify_routes, user_routes::user_routes};

/**
A function that returns a `Router` instance

Contains all the routes that we handle using either `handle_get` or `handle_post`
 */
pub fn get_routes() -> Router {
    Router::new()
        .merge(user_routes())
        .merge(spotify_routes())
        .route("/", get(handle_get::index()))
        .fallback(handle_get::index())
}
