use rand::random;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use tokio::task;
use bytes::BytesMut;
use tokio_postgres::{
    Client,
    types::{FromSql, ToSql, IsNull, Type, to_sql_checked}
};

use std::error::Error;
use std::fmt::{ Formatter, Display, Debug};
use std::fmt;
use std::env;
use std::sync::Arc;
use std::time::Duration;

use crate::User;

pub struct SpotifyLoginError {
    kind: String,
    message: String
}

// Implement std::fmt::Display for LoginError
impl Display for SpotifyLoginError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message) // user-facing output
    }
}

// Implement std::fmt::Debug for LoginError
impl Debug for SpotifyLoginError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
//Use the fancy 'a this way you don't use the heap (idk how this works lol)
#[derive(Debug, Serialize, Deserialize)]
struct SpotifyCodeRequest<'a> {
    response_type: &'a str,
    client_id: &'a str,
    scope: &'a str,
    redirect_uri: &'a str,
    state: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpotifyCode {
    code: Option<String>,
    error: Option<String>,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpotifyRequestToken<'a> {
    grant_type: &'a str,
    code: &'a str,
    redirect_uri: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpotifyToken {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: i32,
    pub refresh_token: String,
}

impl<'a> FromSql<'a> for SpotifyToken {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<SpotifyToken, Box<(dyn Error + Send + Sync + 'static)>> {
        let json_str = std::str::from_utf8(raw)?;
        let token = serde_json::from_str(json_str)?;
        Ok(token)
    }

    fn accepts(ty: &Type) -> bool {
        *ty == Type::JSONB
    }

    fn from_sql_null(ty: &Type) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Err(Box::new(tokio_postgres::types::WasNull))
    }

    fn from_sql_nullable(
        ty: &Type,
        raw: Option<&'a [u8]>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match raw {
            Some(raw) => Self::from_sql(ty, raw),
            None => Self::from_sql_null(ty),
        }
    }
}

impl ToSql for SpotifyToken {
    fn to_sql(&self, _: &Type, out: &mut BytesMut) -> Result<IsNull, Box<(dyn Error + Send + Sync + 'static)>> {
        let json_str = serde_json::to_string(self)?;
        out.extend_from_slice(json_str.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        *ty == Type::JSONB
    }

    to_sql_checked!();
}

/*
key	            Type	Description
access_token	string	An access token that can be provided in subsequent calls, for example to Spotify Web API services.
token_type	    string	How the access token may be used: always "Bearer".
scope	        string	A space-separated list of scopes which have been granted for this access_token
expires_in	    int	    The time period (in seconds) for which the access token is valid.
refresh_token	string	See refreshing tokens.
*/

pub async fn spotify_auth(client: Arc<Client>, user_id: i32) -> Result<SpotifyToken, SpotifyLoginError> {
    let user: User = match find_authenticated_user(&client, &user_id).await {
        Some(u) => u,
        None => return Err(SpotifyLoginError {
            kind: "UserNotFound".to_string(),
            message: format!("Failed to find user with id: {}", &user_id),
        })
    };

    if let Some(spotifytoken) = user.spotifytoken {
        return Ok(spotifytoken);
    }

    let token = match spotify_login(&user).await {
        Ok(t) => t,
        Err(e) => return Err(e)
    };
    match add_spotify_token(&client, user_id, token.clone()).await {
        Ok(_) => {},
        Err(e) => return Err(e)
    }
    Ok(token)
}

async fn find_authenticated_user(client: &Client, user_id: &i32) -> Option<User> {
    let stmt = match  client.prepare(
        "SELECT id, username, password, salt, email, spotifytoken, liked_songs, disliked_songs
        FROM users
        WHERE id = $1"
    ).await {
        Ok(q) => q,
        Err(_) => return None
    };

    match client.query_one(&stmt, &[&user_id]).await {
        Ok(row) => {
            Some(User {
            id: row.get("id"),
            username: row.get("username"),
            password: row.get("password"),
            salt: row.get("salt"),
            email: row.get("email"),
            spotifytoken: row.get("spotifytoken"),
            liked_songs: row.get("liked_songs"),
            disliked_songs: row.get("disliked_songs"),
            })
        },
        Err(_) => None
    }
}

async fn spotify_login(user: &User) -> Result<SpotifyToken, SpotifyLoginError> {
    match spotify_token().await {
        Ok(t) => Ok(t),
        Err(e) => Err(e)
    }
}

async fn add_spotify_token(client: &Client, user_id: i32, token: SpotifyToken) -> Result<(), SpotifyLoginError> {
    let stmt = match client.prepare(
        "UPDATE users
        SET spotifytoken = $1
        WHERE id = $2"
    ).await {
        Ok(q) => q,
        Err(_) => return Err(SpotifyLoginError {
            kind: "SpotifyTokenQueryFail".to_string(),
            message: "Failed to create a spotify token query".to_string()
        })
    };

    match client.execute(&stmt, &[&token, &user_id]).await {
        Ok(_) => Ok(()),
        Err(_) => Err(SpotifyLoginError {
            kind: "AddSpotifyTokenFailure".to_string(),
            message: "Failed to add spotify token to the database".to_string()
        })
    }
}

fn start_token_refresh_task(client: Client, user_id: i32, token: SpotifyToken) {
    // Calculate the sleep duration, subtracting a buffer (e.g., 60 seconds)
    let refresh_duration = Duration::from_secs((token.expires_in - 60) as u64);

    task::spawn(async move {
        tokio::time::sleep(refresh_duration).await;
        if let Err(e) = refresh_token(client, user_id, token).await {
            eprintln!("Failed to refresh token: {}", e);
        }
    });
}

async fn refresh_token(client: Client, user_id: i32, token: SpotifyToken) -> Result<(), reqwest::Error> {
    let spotify_client_id = env::var("SPOTIFY_CLIENT_ID").expect("You must set the SPOTIFY_CLIENT_ID env var!");

    let request_client = reqwest::Client::new();
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", &token.refresh_token),
        ("client_id", &spotify_client_id)
    ];

    let response = request_client
        .post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        drop(token);
        let new_token: SpotifyToken = response.json().await?;
        match add_spotify_token(&client, user_id, new_token).await {
            Ok(_) => {},
            Err(_) => eprintln!("Failed to added refreshed token")
        };
    } else {
        eprintln!("Failed to refresh token: {}", response.status());
    }

    Ok(())
}

//Don't exactly understand how this works lmao just copy-pasted this shit
fn generate_state(length: u32) -> String {
    (0..length)
        .map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char)
        .collect()
}

async fn spotify_code() -> Result<SpotifyCode, reqwest::Error> {
    let state: String = generate_state(16);
    let spotify_client_id: String = env::var("SPOTIFY_CLIENT_ID").expect("You must set the SPOTIFY_CLIENT_ID env var!");
    let scope: &str = "user-read-playback-state user-modify-playback-state";

    let auth_query_parameters = SpotifyCodeRequest {
        response_type: "code",
        client_id: &spotify_client_id,
        scope: scope,
        redirect_uri: "https://localhost:8080/api/v1/",
        state: &state,
    };

    let request_client = reqwest::Client::new();
    let response = request_client
        .get("https://accounts.spotify.com/authorize")
        .query(&auth_query_parameters)
        .send()
        .await;
    match response {
        Ok(response) => {
            let code: Result<SpotifyCode, _> = response.json().await;
            code
        }
        Err(err) => Err(err),
    }
}

async fn spotify_token() -> Result<SpotifyToken, SpotifyLoginError> {
    let code: String = match spotify_code().await {
        Ok(response) => {
            if let Some(code) = response.code {
                code
            } else {
                //Fuck you
                return Err(SpotifyLoginError{
                    kind: "SpotifyCodeError".to_string(),
                    message: "Get request to spotify API connected but failed".to_string()
                });
            }
        }
        Err(err) => return Err(SpotifyLoginError{
            kind: "ReqwestCodeError".to_string(),
            message: err.to_string()
        }),
    };


    let form_data = SpotifyRequestToken {
        grant_type: "code",
        code: &code,
        redirect_uri: "https://localhost:8081/api/v1/",
    };
    let auth_header_value = auth_header();
    let request_client = reqwest::Client::new();
    let response = request_client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", auth_header_value)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form_data)
        .send()
        .await;

    let token: Result<SpotifyToken, reqwest::Error> = match response {
        Ok(t) => {
            t.json().await
        },
        Err(err) => return Err(SpotifyLoginError{
            kind: "SpotifyTokenError".to_string(),
            message: err.to_string()
        }),
    };

    match token {
        Ok(t) => Ok(t),
        Err(e) => Err(SpotifyLoginError{
            kind: "SpotifyTokenConversionError".to_string(),
            message: e.to_string()
        }),
    }
}

fn auth_header() -> String {
    let spotify_client_id: String = env::var("SPOTIFY_CLIENT_ID").expect("You must set the SPOTIFY_CLIENT_ID env var!");
    let spotify_client_secret: String = env::var("SPOTIFY_CLIENT_SECRET").expect("You must set the SPOTIFY_CLIENT_SECRET env var!");
    let mut auth_header_value = String::new();
    general_purpose::STANDARD.encode_string(&format!("{}:{}", spotify_client_id, spotify_client_secret), &mut auth_header_value);
    format!(
        "Basic {}",
        auth_header_value
    )
}
