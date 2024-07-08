use serde::{Serialize, Deserialize};

// You use `serde` to create structs which can serialize & deserialize between BSON:
#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
   id: i32,
   uri: String,
}
