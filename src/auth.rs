use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
    pub roles: String,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: i32, roles: String, expires_in: Duration) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id,
            exp: (now + expires_in).timestamp(),
            roles,
            iat: now.timestamp(),
        }
    }
}

#[derive(Clone)]
pub struct JwtService {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub access_token_till: Duration,
}

impl JwtService {
    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}
