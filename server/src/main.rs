use axum::{
    Router,
    extract::Extension
};
use std::error::Error;
use std::env;
use std::sync::Arc;

use tokio_postgres::NoTls;

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

    let database_url = env::var("DATABASE_URL").expect("Must set DATABASE_URL!");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await.unwrap();

    let app = Router::new().merge(get_routes()).layer(Extension(Arc::new(client)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
