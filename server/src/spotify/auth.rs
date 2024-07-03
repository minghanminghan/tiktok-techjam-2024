use serde::{ Serialize, Deserialize};
use rand::random;
use std::collections::HashMap;
use std::env;
use reqwest::Client;

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
    state: String
}

#[derive(Debug, Serialize, Deserialize)]
struct SpotifyRequestToken<'a> {
    grant_type: &'a str,
    code: &'a str,
    redirect_uri: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyToken {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: i32,
    refresh_token: String
}


/*
key	            Type	Description
access_token	string	An access token that can be provided in subsequent calls, for example to Spotify Web API services.
token_type	    string	How the access token may be used: always "Bearer".
scope	        string	A space-separated list of scopes which have been granted for this access_token
expires_in	    int	    The time period (in seconds) for which the access token is valid.
refresh_token	string	See refreshing tokens.
*/

//Don't exactly understand how this works lmao just copy-pasted this shit
fn generate_state(length: u32) -> String {
    (0..length).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect()
}

async fn spotify_code() -> Result<String, reqwest::Error> {
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

    let client = Client::new();
    match client
        .get("https://accounts.spotify.com/authorize")
        .query(&auth_query_parameters)
        .send()
        .await {
        Ok(response) => response.text().await,
        Err(err) => Err(err),
    }
}

async fn spotify_token() {

}
