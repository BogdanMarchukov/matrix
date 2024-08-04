use std::usize;
use crate::config;
use chrono::Utc;
use jsonwebtoken::{encode, errors, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub fn create_hash_sha256(input_str: &String, salt: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input_str.as_bytes());
    hasher.update(salt.as_bytes());
    let hash: String = format!("{:X}", hasher.finalize());
    hash
}

#[derive(Debug, Deserialize, Serialize)]
struct JwtPayload {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &String) -> Result<String, errors::Error> {
    let secret = config::get_jwt_sectet();
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(3))
        .expect("valid timestamp")
        .timestamp();
    let payload = JwtPayload {
        sub: user_id,
        exp: expiration as usize,
    };
    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(&secret.as_ref()),
    );
    token
}

#[test]
fn check_create_hash() {
    let salt = String::from("salt");
    let input_one = String::from("foo");
    let second = String::from("foo");
    let one_hash = create_hash_sha256(&input_one, &salt);
    let second_hash = create_hash_sha256(&second, &salt);
    assert_eq!(one_hash, second_hash)
}
