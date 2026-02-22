use std::{env, string};

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaim {
    pub sub: i32, // user ID
    pub username: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionClaim {
    pub sub: i32,
    pub exp: usize,
    // NOTE: If you are wondering about user agent, stop bcs i'll never put this
}

impl AccessClaim {
    pub fn generate(id: i32, username: String) -> String {
        let secret = env::var("JWT_KEY").expect("JWT_KEY must be set");
        let claims = Self {
            sub: id,
            username: username,
            exp: (Utc::now() + Duration::days(3)).timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }

    pub fn verify(token: String) -> Result<Self, String> {
        let secret = env::var("JWT_KEY").expect("JWT_KEY must be set");
        match decode(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
            Ok(t) => Ok(t.claims),
            Err(e) => {
                println!("{:?}", e);
                Err(String::from(format!("{}", e)))
            }
        }
    }
}

impl SessionClaim {
    pub fn generate(user_id: i32) -> String {
        let secret = env::var("JWT_KEY").expect("JWT_KEY must be set");
        let claims = Self {
            sub: user_id,
            exp: (Utc::now() + Duration::days(365 * 20)).timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }

    pub fn verify(token: String) -> Result<Self, String> {
        let secret = env::var("JWT_KEY").expect("JWT_KEY must be set");
        match decode(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
            Ok(t) => Ok(t.claims),
            Err(e) => {
                println!("{:?}", e);
                Err(String::from(format!("{}", e)))
            }
        }
    }
}
