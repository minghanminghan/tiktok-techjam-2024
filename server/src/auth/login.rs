use bcrypt::verify;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::User;

pub struct LoginError {
    kind: String,
    message: String,
}

// Implement std::fmt::Display for LoginError
impl Display for LoginError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message) // user-facing output
    }
}

// Implement std::fmt::Debug for LoginError
impl Debug for LoginError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub async fn login(
    pool: &PgPool,
    identifier: &str,
    password: &str,
) -> Result<String, LoginError> {
    let user: User = match find_user(pool, identifier).await {
        Ok(u) => u,
        Err(err) => return Err(err),
    };

    let authenticated: bool = match authenticate_user(password, &user.password) {
        Ok(a) => a,
        Err(err) => return Err(err),
    };

    if !authenticated {
        return Err(LoginError {
            kind: "IncorrectPassword".to_string(),
            message: "Wrong password".to_string(),
        });
    }

    let token: String = match generate_jwt(&user.id.to_string()) {
        Ok(t) => t,
        Err(_) => {
            return Err(LoginError {
                kind: "FailedTokenGeneration".to_string(),
                message: "Failed to generate JWT token".to_string(),
            })
        }
    };

    Ok(token)
}

async fn find_user(pool: &PgPool, identifier: &str,) -> Result<User, LoginError> {
    match sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password, salt, email, spotifytoken, liked_songs, disliked_songs
        FROM users
        WHERE username = $1 OR email = $1
        "#,
        identifier
    ).fetch_optional(pool).await {
        Ok(u) => Ok(u),
        Err(e) => Err(LoginError {
                    kind: "UserNotFound".to_string(),
                    message: "Could not find user based on given identifier".to_string(),
        })
    }
}

fn authenticate_user(password: &str, hash: &str) -> Result<bool, LoginError> {
    match verify(password, hash) {
        Ok(v) => Ok(v),
        Err(err) => Err(LoginError {
            kind: "LoginAttemptFailed".to_string(),
            message: "Internal login attempt failure".to_string(),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // The subject of the token (typically the user ID)
    exp: usize,  // Expiration time (as seconds since the epoch)
}

fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize
        + 3600; // Token expiration time (e.g., 1 hour from now)

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    let private_key = env::var("JWT_PRIVATE_KEY").expect("JWT_PRIVATE_KEY env variable not found");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(private_key.as_ref()),
    )?;
    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // Get the public key from the environment
    let public_key = env::var("JWT_PUBLIC_KEY").expect("JWT_PUBLIC_KEY env variable not found");

    // Define the validation parameters
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;

    // Decode and validate the token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(public_key.as_ref()),
        &validation,
    )?;

    Ok(token_data.claims)
}
