use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
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

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_serialize() {
        let u = User {
            id: 42,
            first_name: Some("Harry".into()),
            last_name: Some("Hwang".into()),
            email: Some("harry@example.com".into()),
            password_digest: Some("$2b$12$hash".into()),
        };
        let json = serde_json::to_value(&u).unwrap();
        assert_eq!(json["id"], 42);
        assert_eq!(json["first_name"], "Harry");
        assert_eq!(json["email"], "harry@example.com");
    }

    #[test]
    fn test_user_deserialize_with_nulls() {
        let json = r#"{"id":1,"first_name":null,"last_name":null,"email":null,"password_digest":null}"#;
        let u: User = serde_json::from_str(json).unwrap();
        assert_eq!(u.id, 1);
        assert!(u.first_name.is_none());
        assert!(u.email.is_none());
    }

    #[tokio::test]
    async fn test_verify_password_correct() {
        let hash = tokio::task::spawn_blocking(|| {
            bcrypt::hash("secret123", 4).unwrap() // cost=4 for fast tests
        })
        .await
        .unwrap();
        assert!(verify_password(&hash, "secret123").await);
    }

    #[tokio::test]
    async fn test_verify_password_wrong() {
        let hash = tokio::task::spawn_blocking(|| {
            bcrypt::hash("secret123", 4).unwrap()
        })
        .await
        .unwrap();
        assert!(!verify_password(&hash, "wrong_password").await);
    }

    #[tokio::test]
    async fn test_verify_password_invalid_hash() {
        assert!(!verify_password("not-a-bcrypt-hash", "anything").await);
    }
}
