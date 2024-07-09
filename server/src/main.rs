use axum::{
    Router,
    extract::Extension
};
use std::error::Error;
use std::env;
use std::sync::Arc;
use dotenv::dotenv;

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
    pub mod fetch_songs;
    pub mod swipe;
}

use routes::routes::get_routes;
use schemas::user::User;
use schemas::song::Song;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Must set DATABASE_URL!");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let app = Router::new().merge(get_routes()).layer(Extension(Arc::new(client)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
