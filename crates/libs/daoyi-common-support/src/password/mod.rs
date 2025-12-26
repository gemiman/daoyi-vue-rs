const PASSWORD_HASH_COST: u32 = 4;
pub async fn hash_password(password: &str) -> anyhow::Result<String> {
    Ok(bcrypt::hash(password, PASSWORD_HASH_COST)?) // bcrypt::DEFAULT_COST
}

pub async fn verify_password(password: &str, hashed_password: &str) -> anyhow::Result<bool> {
    Ok(bcrypt::verify(password, hashed_password)?)
}

#[tokio::test]
async fn test_hash_password() {
    let password = "Aa123456";
    let hashed_password = hash_password(password).await.unwrap();
    println!("Hashed password: {}", &hashed_password);
}