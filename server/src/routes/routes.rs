use axum::{
    routing::get,
    Router
};
use mongodb::Collection;

use crate::routes::handle_get;
use crate::routes::{spotify_routes::spotify_routes, user_routes::user_routes};
use crate::User;
use crate::Song;
use crate::AppState;

/**
A function that returns a `Router` instance

Contains all the routes that we handle using either `handle_get` or `handle_post`
 */
pub fn get_routes(appstate: AppState) -> Router {
    Router::new()
        .merge(user_routes(appstate.clone()))
        .merge(spotify_routes(appstate.clone()))
        .route("/", get(handle_get::index()))
        .fallback(handle_get::index())
}
