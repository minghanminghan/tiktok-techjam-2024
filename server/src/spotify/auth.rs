use serde::{ Serialize, Deserialize};

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
