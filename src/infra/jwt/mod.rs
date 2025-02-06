use std::env;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use crate::domain::errors::domain_error::DomainError;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn generate_jwt(user_id: Uuid) -> Result<String, DomainError> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let expiration_hours: i64 = env::var("JWT_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse()
        .unwrap_or(24);

    let exp = Utc::now()
        .checked_add_signed(Duration::hours(expiration_hours))
        .unwrap()
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: exp as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| DomainError::Infra(format!("JWT encode error: {:?}", e)))
}

pub fn validate_jwt(token: &str) -> Result<Claims, DomainError> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let validation = Validation::default();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map_err(|_| DomainError::Unauthorized("Invalid token".to_string()))
    .map(|data| data.claims)
}
