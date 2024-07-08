use axum::{
    Router,
    extract::Extension
};
use std::error::Error;
use std::env;
use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;

mod routes { 
    pub mod routes;
    pub mod handle_get;
    pub mod user_routes;
    pub mod spotify_routes;
}

mod schemas {
    pub mod user;
    pub mod song;
}

mod auth {
    pub mod login;
    pub mod register;
}

mod spotify {
    pub mod auth;
    pub mod player;
}

use routes::routes::get_routes;
use schemas::user::User;
use schemas::song::Song;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let postgres_uri = env::var("POSTGRES_URI").expect("Must set POSTGRES_URI!");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_uri)
        .await?;

    let app = Router::new().merge(get_routes()).layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
