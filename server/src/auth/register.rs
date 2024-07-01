use bcrypt::{hash_with_result, BcryptError};
use mongodb::{bson::{doc, oid}, Collection};
use crate::User;
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

pub async fn register(user_collection: Arc<Collection<User>>, username: &str, email: &str, password: &str) -> Result<User, RegistrationError> {
    if !is_avaliable(user_collection.clone(), username, email).await {
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

    let new_user = User {
        _id: oid::ObjectId::from_str("id_str").unwrap(),
        username: username.to_string(),
        password: hash,
        salt: salt,
        spotifytoken: None,
        email: email.to_string(),
        liked: Vec::new(),
        disliked: Vec::new(),
    };

    match user_collection.insert_one(&new_user).await {
        Ok(_) => Ok(new_user),
        Err(_) => Err(RegistrationError{
            kind: "UserInsertionError".to_string(),
            message: "Failed to insert user into MongoDB".to_string(),
        }),
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

async fn is_avaliable(user_collection: Arc<Collection<User>>, username: &str, email: &str) -> bool {
    let filter = doc! {
        "$or": [
            { "username": username },
            { "email": email }
        ]
    };

    let result = user_collection.find_one(filter).await;
    match result {
        Ok(user) => user.is_some(),
        Err(_err) => false,
    }
}
