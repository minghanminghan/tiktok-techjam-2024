use bcrypt::{hash_with_result, BcryptError};
use tokio_postgres::Client;

use crate::User;
use crate::schemas::user::NewUser;

use std::fmt::{Display, Debug, Formatter};
use std::fmt;
use std::str::FromStr;
use std::env;
use std::sync::Arc;


pub struct RegistrationError {
    kind: String,
    message: String,
}

// Implement std::fmt::Display for RegistrationError
impl Display for RegistrationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message) // user-facing output
    }
}

// Implement std::fmt::Debug for RegistrationError
impl Debug for RegistrationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub async fn register(client: &Client, username: &str, email: &str, password: &str) -> Result<User, RegistrationError> {
    if is_avaliable(client, username, email).await {
        return Err(RegistrationError{
            kind: "UsernameEmailInUse".to_string(),
            message: "The provided username or email is already in use".to_string()
        });
    }

    let hash: String;
    let salt: String;
    match generate_hash(password) {
        Ok(h) => (hash, salt) = h,
        Err(_) => return Err(RegistrationError{
            kind: "HashGenerationFailed".to_string(),
            message: "Failed to generate hash".to_string()
        })
    };

    let new_user: NewUser = NewUser {
        username: username.to_string(),
        email: email.to_string(),
        password: hash,
        salt: salt
    };

    create_user(client, new_user).await
}

pub async fn create_user(client: &Client, new_user: NewUser) -> Result<User, RegistrationError> {
    let stmt = match client.prepare(
        "INSERT INTO users (username, password, salt, email)
        VALUES ($1, $2, $3, $4)
        RETURNING id"
    ).await {
        Ok(q) => q,
        Err(_) => return Err(RegistrationError {
            kind: "UserCreationFailed".to_string(),
            message: "Failed to make the user into a SQL query".to_string()
        })
    };

    match client.query_one(&stmt, &[&new_user.username, &new_user.password, &new_user.salt, &new_user.email]).await {
        Ok(row) => Ok(row.get("id")),
        Err(_) => Err(RegistrationError {
            kind: "UserIdFetchFailed".to_string(),
            message: "Failed to fetch the user id".to_string()
        })

    }
}

/**
 * Returns Result<(hash, salt), BcryptErro>
 */
fn generate_hash(password: &str) -> Result<(String, String), BcryptError> {
    match hash_with_result(password, 12) {
        Ok(hash) => Ok((hash.to_string(), hash.get_salt())),
        Err(e) => Err(e),
    }
}

async fn is_avaliable(client: &Client, username: &str, email: &str) -> bool {
    let stmt = match client.prepare(
        "SELECT EXISTS (
            SELECT 1
            FROM users
            WHERE username = $1 OR email = $2
        )"
    ).await {
        Ok(q) => q,
        Err(_) => return false
    };

    match client.query_one(&stmt, &[&username, &email]).await {
        Ok(row) => row.get(0),
        Err(_) => false
    }
}
