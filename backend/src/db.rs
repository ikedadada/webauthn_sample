use sqlx::{MySqlPool, Row};
use crate::models::{User, Credential};

pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect(database_url).await
}

pub async fn create_user(pool: &MySqlPool, username: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO users (username) VALUES (?)")
        .bind(username)
        .execute(pool)
        .await?;
    
    Ok(result.last_insert_id() as i64)
}

pub async fn get_user_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT id, username FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    
    Ok(user)
}

pub async fn save_credential(
    pool: &MySqlPool,
    user_id: i64,
    credential_id: &[u8],
    public_key: &[u8],
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO credentials (user_id, credential_id, public_key, sign_count) VALUES (?, ?, ?, 0)")
        .bind(user_id)
        .bind(credential_id)
        .bind(public_key)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn get_credentials_by_user_id(pool: &MySqlPool, user_id: i64) -> Result<Vec<Credential>, sqlx::Error> {
    let credentials = sqlx::query_as::<_, Credential>(
        "SELECT id, user_id, credential_id, public_key, sign_count FROM credentials WHERE user_id = ?"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    
    Ok(credentials)
}

pub async fn update_sign_count(
    pool: &MySqlPool,
    credential_id: &[u8],
    sign_count: u32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE credentials SET sign_count = ? WHERE credential_id = ?")
        .bind(sign_count as i64)
        .bind(credential_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn store_challenge(
    pool: &MySqlPool,
    session_id: &str,
    challenge_type: &str,
    challenge_data: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO challenge_storage (session_id, challenge_type, challenge_data, expires_at) VALUES (?, ?, ?, DATE_ADD(NOW(), INTERVAL 5 MINUTE))")
        .bind(session_id)
        .bind(challenge_type)
        .bind(challenge_data)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn get_challenge(
    pool: &MySqlPool,
    session_id: &str,
    challenge_type: &str,
) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query("SELECT challenge_data FROM challenge_storage WHERE session_id = ? AND challenge_type = ? AND expires_at > NOW()")
        .bind(session_id)
        .bind(challenge_type)
        .fetch_optional(pool)
        .await?;
    
    Ok(row.map(|r| r.get("challenge_data")))
}

pub async fn delete_challenge(
    pool: &MySqlPool,
    session_id: &str,
    challenge_type: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM challenge_storage WHERE session_id = ? AND challenge_type = ?")
        .bind(session_id)
        .bind(challenge_type)
        .execute(pool)
        .await?;
    
    Ok(())
}