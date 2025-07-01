use crate::config;
use async_graphql::Error;
use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::usize;

pub fn _create_hash_sha256(input_str: &String, salt: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input_str.as_bytes());
    hasher.update(salt.as_bytes());
    let hash: String = format!("{:X}", hasher.finalize());
    hash
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtPayload {
    pub sub: String,
    pub exp: i64,
}

pub fn create_jwt(user_id: String) -> Result<String, errors::Error> {
    let secret = config::get_jwt_sectet();
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(3))
        .expect("valid timestamp")
        .timestamp();
    let payload = JwtPayload {
        sub: user_id,
        exp: expiration,
    };
    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_ref()),
    );
    token
}

pub fn verify_jwt(token: String) -> Result<TokenData<JwtPayload>, Error> {
    let secret = config::get_jwt_sectet();
    let result = decode::<JwtPayload>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    let jwt_payload = match result {
        Ok(v) => v,
        Err(err) => return Err(Error::new(format!("invalid token: {}", err))),
    };
    let curremt_timestamp = Utc::now().timestamp();
    let exp_date = &jwt_payload.claims.exp;
    if exp_date < &curremt_timestamp {
        return Err(Error::new("expire token"));
    }
    Ok(jwt_payload)
}

#[test]
fn check_create_hash() {
    let salt = String::from("salt");
    let input_one = String::from("foo");
    let second = String::from("foo");
    let one_hash = _create_hash_sha256(&input_one, &salt);
    let second_hash = _create_hash_sha256(&second, &salt);
    assert_eq!(one_hash, second_hash)
}
