use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

use crate::spotify::auth::SpotifyToken;
use crate::schemas::song::Song;

// You use `serde` to create structs which can serialize & deserialize between BSON:
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
   #[serde(rename = "_id")]
   pub _id: ObjectId,
   pub username: String,
   pub password: String,
   pub salt: String,
   pub email: String,
   pub spotifytoken: Option<SpotifyToken>,
   pub liked: Vec<Song>,
   pub disliked: Vec<Song>
}
