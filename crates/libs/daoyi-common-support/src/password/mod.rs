const PASSWORD_HASH_COST: u32 = 4;
pub async fn hash_password(password: &str) -> anyhow::Result<String> {
    Ok(bcrypt::hash(password, PASSWORD_HASH_COST)?) // bcrypt::DEFAULT_COST
}

pub async fn verify_password(password: &str, hashed_password: &str) -> anyhow::Result<bool> {
    Ok(bcrypt::verify(password, hashed_password)?)
}
