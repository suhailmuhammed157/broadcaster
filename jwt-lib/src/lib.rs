use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Platform {
    pub platform_id: i64,
    pub platform_name: String,
}

#[derive(Deserialize, Serialize)]
struct Claims {
    platform_id: i64,
    platform_name: String,
    exp: i64,
}

pub fn get_jwt(platform: Platform) -> Result<String, String> {
    let token = encode(
        &Header::default(),
        &Claims {
            platform_id: platform.platform_id,
            platform_name: platform.platform_name,
            exp: (Utc::now() + Duration::minutes(1)).timestamp(),
        },
        &EncodingKey::from_secret("mykey".as_bytes()),
    )
    .map_err(|e| e.to_string());

    return token;
}

pub fn decode_jwt(token: &str) -> Result<Platform, String> {
    let token_data = decode::<Platform>(
        token,
        &DecodingKey::from_secret("mykey".as_bytes()),
        &Validation::default(),
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),

        Err(e) => Err(e.to_string()),
    }
}
