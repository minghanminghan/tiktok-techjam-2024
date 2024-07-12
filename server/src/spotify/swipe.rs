use tokio_postgres::Client;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub struct SwipeError {
    kind: String,
    message: String,
}

// Implement std::fmt::Display for SwipeError
impl Display for SwipeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message) // user-facing output
    }
}

// Implement std::fmt::Debug for SwipeError
impl Debug for SwipeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub async fn like_song(client: &Client, user_id: i32, song_id: i32) -> Result<(), SwipeError> {
    let stmt = match client.prepare(
        "UPDATE users
        SET liked_songs = ARRAY_APPEND(COALESCE(liked_songs, ARRAY[]::INTEGER[]), $1)
        WHERE id = $2"
    ).await {
        Ok(q) => q,
        Err(_) => return Err(SwipeError {
            kind:"QueryCreationFailed".to_string(),
            message:"Failed to create query to like song".to_string()
        })
    };

    match client.execute(&stmt, &[&song_id, &user_id]).await {
        Ok(_) => Ok(()),
        Err(_) => Err(SwipeError {
            kind: "InsertLikedSongFail".to_string(),
            message: "Failed to insert liked song to user".to_string()
        })
    }
}

