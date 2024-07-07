use axum::Router;
use mongodb::{
    Client,
    options::ClientOptions,
    Collection
};
use std::error::Error;
use std::env;
use std::sync::Arc;

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

#[derive(Clone)]
pub struct AppState {
    user_collection: Arc<Collection<User>>,
    song_collection: Arc<Collection<Song>>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options = ClientOptions::parse(&client_uri).await?;
    let client = Client::with_options(options)?;
    let db = client.database("techjam");

    let user_collection: mongodb::Collection<User> = db.collection("users");
    let song_collection: mongodb::Collection<Song> = db.collection("songs");

    let appstate: AppState = AppState {
        user_collection: Arc::new(user_collection),
        song_collection: Arc::new(song_collection)
    };
    let app = Router::new().merge(get_routes(appstate));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
