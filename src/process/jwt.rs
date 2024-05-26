use crate::utils::read_input;
use anyhow::Result;
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    exp: usize,
}

pub async fn process_jwt_sign(input: &str, key: &str, aud: &str) -> Result<String> {
    let buf = read_input(input)?;
    let claims = Claims {
        sub: String::from_utf8(buf).unwrap(),
        exp: 1000000000,
        aud: aud.to_string(),
    };
    // 准备加密 key
    let encoding_key = EncodingKey::from_secret(key.as_bytes());
    // 生成 token
    let token = jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)?;
    Ok(token)
}

pub async fn process_jwt_verify(input: &str, key: &str) -> Result<bool> {
    let buf = read_input(input)?;
    let token = String::from_utf8(buf)?;
    let encoding_key = DecodingKey::from_secret(key.as_bytes());
    let _ = jsonwebtoken::decode::<Claims>(
        &token,
        &encoding_key,
        &jsonwebtoken::Validation::default(),
    )?;
    Ok(true)
}
