use crate::model::user::{Credential, User};
use argon2::{password_hash::{Error, SaltString, rand_core::OsRng, PasswordHasher as ArgonPasswordHasher}, Argon2, PasswordHash, PasswordVerifier};
use rand::{distributions::Alphanumeric, Rng};

pub fn authorize_user(user: &User, credentials: Credential) -> Result<String, Error> {
    let argon2 = Argon2::default();
    let db_hash = PasswordHash::new(&user.password)?;
    argon2.verify_password(credentials.password.as_bytes(), &db_hash)?;

    let session_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
            .take(128)
            .map(char::from)
            .collect();

    Ok(session_id)
}

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    let argon2 = argon2::Argon2::default();
    
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}