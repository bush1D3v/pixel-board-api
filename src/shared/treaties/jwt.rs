use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct Jwt;

impl Jwt {
    pub fn access_token(user_id: &str) -> Result<(String, i64), String> {
        let expires_in: i64 = 30 * 60; // 30 minutes
        let secret = env::var("JWT_ACCESS_KEY").map_err(|_| "JWT_ACCESS_KEY not set".to_string())?;
        let now = Utc::now().timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + expires_in as usize,
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| format!("Failed to create access token: {e}"))?;

        Ok((token, expires_in))
    }

    pub fn refresh_token(user_id: &str) -> Result<(String, i64), String> {
        let expires_in: i64 = 7 * 24 * 60 * 60; // 7 days
        let secret =
            env::var("JWT_REFRESH_KEY").map_err(|_| "JWT_REFRESH_KEY not set".to_string())?;
        let now = Utc::now().timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + expires_in as usize,
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| format!("Failed to create refresh token: {e}"))?;

        Ok((token, expires_in))
    }

    pub fn validate_access_token(token: &str) -> Result<Claims, String> {
        let secret = env::var("JWT_ACCESS_KEY").map_err(|_| "JWT_ACCESS_KEY not set".to_string())?;
        Self::decode_token(token, &secret)
    }

    pub fn validate_refresh_token(token: &str) -> Result<Claims, String> {
        let secret =
            env::var("JWT_REFRESH_KEY").map_err(|_| "JWT_REFRESH_KEY not set".to_string())?;
        Self::decode_token(token, &secret)
    }

    fn decode_token(token: &str, secret: &str) -> Result<Claims, String> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| format!("Invalid token: {e}"))
    }
}
