use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::spotify::auth::SpotifyToken;
use crate::schemas::song::Song;

// You use `serde` to create structs which can serialize & deserialize between BSON:
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
   pub id: i32,
   pub username: String,
   pub password: String,
   pub salt: String,
   pub email: String,
   pub spotifytoken: Option<SpotifyToken>,
   pub liked: Vec<i32>,
   pub disliked: Vec<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: String,
}
