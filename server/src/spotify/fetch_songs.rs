use std::path::Path;
use pyo3::{
    prelude::*,
    types::{PyModule, PyList}
};
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use tokio_postgres::Client;

use crate::User;
use crate::Song;

pub struct FetchSongError {
    kind: String,
    message: String,
}

// Implement std::fmt::Display for FetchSongError
impl Display for FetchSongError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message) // user-facing output
    }
}

// Implement std::fmt::Debug for FetchSongError
impl Debug for FetchSongError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub async fn fetch_songs(client: &Client, user_id: i32) -> Result<Vec<String>, FetchSongError>{
    let stmt = match client.prepare(
        "SELECT id, username, password, salt, email, spotifytoken, liked_songs, disliked_songs
        FROM users
        WHERE id = $1"
    ).await {
        Ok(q) => q,
        Err(_) => return Err(FetchSongError{
            kind: "UserNotFound".to_string(),
            message: "Could not find user based on given identifier".to_string(),
        })
    };

    let user = match client.query_one(&stmt, &[&user_id]).await {
        Ok(row) => {
            User {
            id: row.get("id"),
            username: row.get("username"),
            password: row.get("password"),
            salt: row.get("salt"),
            email: row.get("email"),
            spotifytoken: row.get("spotifytoken"),
            liked_songs: row.get("liked_songs"),
            disliked_songs: row.get("disliked_songs"),
            }
        },
        Err(_) => return Err(FetchSongError{
            kind: "UserNotFound".to_string(),
            message: "Could not find user based on given identifier".to_string(),
        })
    };
    if let Some(liked) = user.liked_songs {
        let reduced_list = last_ten_items(&liked);
        match call_python(Some(reduced_list)) {
            Ok(r) => Ok(r),
            Err(_) => Err(FetchSongError {
                kind: "PythonError".to_string(),
                message: "Python function failed to run".to_string()
            }),
        }
    } else {
    match call_python(user.liked_songs) {
        Ok(r) => Ok(r),
        Err(_) => Err(FetchSongError {
            kind: "PythonError".to_string(),
            message: "Python function failed to run".to_string()
        }),
    }
    }
}

fn call_python(liked_songs: Option<Vec<i32>>) -> Result<Vec<String>, PyErr>{
    let path = Path::new("~/tiktok-techjam-2024/python");
    let code: String = std::fs::read_to_string(path).unwrap();
    Python::with_gil(|py| -> Result<Vec<String>, PyErr> {
        let module = match PyModule::from_code_bound(py, &code, "", "") {
            Ok(module) => module,
            Err(err) => return Err(err),
        };

        let function = match module.getattr("") {
            Ok(f) => f,
            Err(err) => return Err(err)
        };
        if let Some(songs) = liked_songs {
            let pylist = PyList::new_bound(py, songs);
            match function.call1((pylist,)) {
                Ok(result) => Ok(result.extract::<Vec<String>>()?),
                Err(err) => Err(err),
            }
        } else {
            match function.call0() {
                Ok(result) => Ok(result.extract::<Vec<String>>()?),
                Err(err) => Err(err),
            }
        }
    })
}

fn last_ten_items<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    let len = vec.len();
    if len > 10 {
        vec[(len - 10)..].to_vec()
    } else {
        vec.clone()
    }
}

async fn song_exists(client: &Client, uri: &str) -> Result<bool, FetchSongError> {
    let stmt = match client.prepare(
        "SELECT 1 FROM songs WHERE uri = $1 LIMIT 1"
    ).await {
        Ok(q) => q,
        Err(_) => return Err(FetchSongError {
            kind: "QueryCreationFailed".to_string(),
            message: "Failed to create query to check if song exists".to_string()
        })
    };

    match client.query_opt(&stmt, &[&uri]).await {
        Ok(row) => Ok(row.is_some()),
        Err(_) => Err(FetchSongError {
            kind: "SongExistsFailed".to_string(),
            message: "Failed to query to check if song exists".to_string()
        })
    }
}

async fn insert_song(client: &Client, new_song: &str) -> Result<(), FetchSongError> {
    let stmt = match client.prepare(
        "INSERT INTO songs (uri) VALUES ($1) RETURNING id, uri"
    ).await {
        Ok(q) => q,
        Err(_) => return Err(FetchSongError{
            kind: "QueryCreationFail".to_string(),
            message: "Failed to create query to insert song".to_string()
        })
    };
    match client.query_one(&stmt, &[&new_song]).await {
        Ok(_) => Ok(()),
        Err(_) => Err(FetchSongError {
            kind: "SongInsertionFail".to_string(),
            message: "Failed to insert song".to_string()
        })
    }
}
