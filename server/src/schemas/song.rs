use serde::{Serialize, Deserialize};
use sqlx::FromRow;

// You use `serde` to create structs which can serialize & deserialize between BSON:
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Song {
   id: i32,
   uri: String,
}
