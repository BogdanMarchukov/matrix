use sha2::{Digest, Sha256};

pub fn create_hash_sha256(input_str: &String, salt: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input_str.as_bytes());
    hasher.update(salt.as_bytes());
    let hash: String = format!("{:X}", hasher.finalize());
    hash
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
