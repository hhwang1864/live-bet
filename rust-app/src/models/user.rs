use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password_digest: Option<String>,
}

pub async fn create_user(
    db: &PgPool,
    first_name: &str,
    last_name: &str,
    email: &str,
    password: &str,
) {
    let password = password.to_string();
    let password_digest = tokio::task::spawn_blocking(move || {
        bcrypt::hash(&password, bcrypt::DEFAULT_COST).unwrap()
    })
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO users(first_name, last_name, email, password_digest) VALUES($1,$2,$3,$4)",
    )
    .bind(first_name)
    .bind(last_name)
    .bind(email)
    .bind(&password_digest)
    .execute(db)
    .await
    .ok();
}

pub async fn find_user_by_email(db: &PgPool, email: &str) -> Option<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(db)
        .await
        .unwrap_or(None)
}

pub async fn find_user_by_id(db: &PgPool, id: i32) -> Option<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(db)
        .await
        .unwrap_or(None)
}

pub async fn verify_password(hash: &str, password: &str) -> bool {
    let hash = hash.to_string();
    let password = password.to_string();
    tokio::task::spawn_blocking(move || bcrypt::verify(&password, &hash).unwrap_or(false))
        .await
        .unwrap_or(false)
}
