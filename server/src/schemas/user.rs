use serde::{Serialize, Deserialize};
use crate::spotify::auth::SpotifyToken;
use bytes::BytesMut;
use tokio_postgres::types::{FromSql, ToSql, IsNull, Type, to_sql_checked};
use std::error::Error;

// You use `serde` to create structs which can serialize & deserialize between BSON:
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
   pub id: i32,
   pub username: String,
   pub password: String,
   pub salt: String,
   pub email: String,
   pub spotifytoken: Option<SpotifyToken>,
   pub liked_songs: Vec<i32>,
   pub disliked_songs: Vec<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: String,
}


impl<'a> FromSql<'a> for User {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<User, Box<(dyn Error + Send + Sync + 'static)>> {
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

impl ToSql for User {
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
