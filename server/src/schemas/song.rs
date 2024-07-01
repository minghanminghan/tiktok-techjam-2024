use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SongType {
    Spotify,
    Uploaded,
}

// You use `serde` to create structs which can serialize & deserialize between BSON:
#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
   #[serde(rename = "_id")]
   id: ObjectId,
   title: String,
   artist: String,
   songtype: SongType,
   release: i32,
}
