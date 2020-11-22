use crate::models::user::User;
use async_graphql::{Enum, Error, Result, SimpleObject};
use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, SimpleObject)]
pub struct Credentials {
    pub access_token: String,
    pub access_token_expires_at: DateTime<Utc>,
    pub refresh_token: String,
    pub refresh_token_expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: uuid::Uuid,
    exp: u64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum)]
pub enum Provider {
    Google,
}

#[derive(Debug, Deserialize)]
struct GoogleIdToken {
    iss: String,
    azp: String,
    aud: String,
    sub: String,
    email: String,
    email_verified: String,
    name: String,
    picture: String,
    given_name: String,
    family_name: String,
    locale: String,
    iat: String,
    exp: String,
    alg: String,
    kid: String,
    typ: String,
}

#[derive(Debug)]
pub struct OAuth2User {
    pub display_name: String,
    pub email: String,
}

pub fn credentials_for_user(
    jwt_secret: &str,
    user: &User,
) -> Result<Credentials> {
    use jsonwebtoken::{encode, EncodingKey, Header};
    let header = Header::default();
    let key = EncodingKey::from_secret(&jwt_secret.as_bytes());
    let access_token_expires_at = Utc::now() + Duration::days(7);
    let refresh_token_expires_at = Utc::now() + Duration::days(30);
    let access_token = encode(
        &header,
        &Claims {
            sub: user.uuid,
            exp: access_token_expires_at.timestamp() as u64,
        },
        &key,
    )
    .map_err(|e| -> Error { e.into() })?;
    let refresh_token = encode(
        &header,
        &Claims {
            sub: user.uuid,
            exp: refresh_token_expires_at.timestamp() as u64,
        },
        &key,
    )
    .map_err(|e| -> Error { e.into() })?;
    Ok(Credentials {
        access_token,
        access_token_expires_at,
        refresh_token,
        refresh_token_expires_at,
    })
}

pub fn extract_bearer_token(authorization_header: &str) -> Option<String> {
    lazy_static! {
        static ref BEARER_REGEX: Regex =
            Regex::new(r"^(B|b)earer\s+(?P<token>[^\s]+)").unwrap();
    }
    BEARER_REGEX.captures(authorization_header).and_then(|cap| {
        cap.name("token").map(|token| token.as_str().to_string())
    })
}

pub fn user_uuid_from_token(jwt_secret: &str, token: &str) -> Result<Uuid> {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims.sub)
    .map_err(|e| -> Error { e.into() })
}

// https://developers.google.com/identity/protocols/oauth2/openid-connect#validatinganidtoken
pub async fn validate_google_id_token(
    google_oauth2_client_id: &str,
    token: &str,
) -> Result<OAuth2User> {
    static INVALID_MSG: &str = "Invalid ID Token";
    // hitting the tokeninfo endpoint will decrypt the token if it is signed with valid google
    // credentials; an alternative would be to download the credentials, since they are rotated
    // infrequently
    let decoded_token: GoogleIdToken = surf::get(&format!(
        "https://oauth2.googleapis.com/tokeninfo?id_token={}",
        token
    ))
    .recv_json()
    .await?;
    // verify that it was issued by valid Google issuer
    if !vec![
        "https://accounts.google.com".to_owned(),
        "accounts.google.com".to_owned(),
    ]
    .contains(&decoded_token.iss)
    {
        return Err(INVALID_MSG.into());
    }
    // verify that the OAuth2 client ID is correct
    if decoded_token.aud != google_oauth2_client_id {
        return Err(INVALID_MSG.into());
    }
    // verify that it has not expired
    let expires_at = decoded_token.exp.parse::<u64>().unwrap_or(0);
    if unixtime() > expires_at {
        return Err(INVALID_MSG.into());
    }
    Ok(OAuth2User {
        display_name: decoded_token.name,
        email: decoded_token.email,
    })
}

pub fn unixtime() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn unixtime_in_n_days(n: u64) -> u64 {
    unixtime() + n * 24 * 60 * 60
}
