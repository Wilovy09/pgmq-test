use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: String) -> Result<String, bcrypt::BcryptError> {
    let hashed_password = hash(password, DEFAULT_COST)?;
    Ok(hashed_password)
}

pub fn verify_password(password: String, hashed_db_password: String) -> bool {
    verify(password, &hashed_db_password).unwrap_or(false)
}
