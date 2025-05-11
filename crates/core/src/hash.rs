use crate::AppResult;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};

pub fn hash_password(pass: &str) -> AppResult<String> {
    let arg2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hashstring = arg2.hash_password(pass.as_bytes(), &salt)?.to_string();

    Ok(hashstring)
}

#[must_use]
pub fn verify_password(pass: &str, hashed_password: &str) -> bool {
    let arg2 = Argon2::default();
    let Ok(hash) = PasswordHash::new(hashed_password) else {
        return false;
    };
    arg2.verify_password(pass.as_bytes(), &hash).is_ok()
}
