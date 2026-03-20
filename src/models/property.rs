use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::search;

const COLS: &str = "id, name, image_url, region, bedroom_no, price";

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Property {
    pub id: i32,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub region: Option<String>,
    pub bedroom_no: Option<i32>,
    pub price: Option<i32>,
}

pub async fn all_properties(db: &PgPool) -> Vec<Property> {
    let q = format!("SELECT {} FROM properties ORDER BY id", COLS);
    sqlx::query_as::<_, Property>(&q)
        .fetch_all(db)
        .await
        .unwrap_or_default()
}

pub async fn get_property(db: &PgPool, id: i32) -> Option<Property> {
    let q = format!("SELECT {} FROM properties WHERE id = $1", COLS);
    sqlx::query_as::<_, Property>(&q)
        .bind(id)
        .fetch_optional(db)
        .await
        .unwrap_or(None)
}

pub async fn create_property(
    db: &PgPool,
    name: &str,
    image_url: &str,
    region: &str,
    bedroom_no: i32,
    price: i32,
) {
    let text = search::property_to_text(name, region, bedroom_no, price);
    let embedding = search::text_to_embedding(&text);
    sqlx::query(
        "INSERT INTO properties(name, image_url, region, bedroom_no, price, embedding)
         VALUES($1,$2,$3,$4,$5,$6)",
    )
    .bind(name)
    .bind(image_url)
    .bind(region)
    .bind(bedroom_no)
    .bind(price)
    .bind(embedding)
    .execute(db)
    .await
    .ok();
}

pub async fn update_property(
    db: &PgPool,
    id: i32,
    name: &str,
    image_url: &str,
    region: &str,
    bedroom_no: i32,
    price: i32,
) {
    let text = search::property_to_text(name, region, bedroom_no, price);
    let embedding = search::text_to_embedding(&text);
    sqlx::query(
        "UPDATE properties
         SET name=$2, image_url=$3, region=$4, bedroom_no=$5, price=$6, embedding=$7
         WHERE id=$1",
    )
    .bind(id)
    .bind(name)
    .bind(image_url)
    .bind(region)
    .bind(bedroom_no)
    .bind(price)
    .bind(embedding)
    .execute(db)
    .await
    .ok();
}

pub async fn delete_property(db: &PgPool, id: i32) {
    sqlx::query("DELETE FROM properties WHERE id = $1")
        .bind(id)
        .execute(db)
        .await
        .ok();
}

pub async fn one_bedroom(db: &PgPool) -> Vec<Property> {
    let q = format!(
        "SELECT {} FROM properties WHERE bedroom_no = 1 ORDER BY id",
        COLS
    );
    sqlx::query_as::<_, Property>(&q)
        .fetch_all(db)
        .await
        .unwrap_or_default()
}

pub async fn two_bedroom(db: &PgPool) -> Vec<Property> {
    let q = format!(
        "SELECT {} FROM properties WHERE bedroom_no = 2 ORDER BY id",
        COLS
    );
    sqlx::query_as::<_, Property>(&q)
        .fetch_all(db)
        .await
        .unwrap_or_default()
}

pub async fn three_bedroom(db: &PgPool) -> Vec<Property> {
    let q = format!(
        "SELECT {} FROM properties WHERE bedroom_no >= 3 ORDER BY id",
        COLS
    );
    sqlx::query_as::<_, Property>(&q)
        .fetch_all(db)
        .await
        .unwrap_or_default()
}

/// Backfill embeddings for any properties that have none.
pub async fn backfill_embeddings(db: &PgPool) {
    let q = format!(
        "SELECT {} FROM properties WHERE embedding IS NULL",
        COLS
    );
    let rows = sqlx::query_as::<_, Property>(&q)
        .fetch_all(db)
        .await
        .unwrap_or_default();

    for p in &rows {
        search::update_embedding(
            db,
            p.id,
            p.name.as_deref().unwrap_or(""),
            p.region.as_deref().unwrap_or(""),
            p.bedroom_no.unwrap_or(0),
            p.price.unwrap_or(0),
        )
        .await;
    }

    if !rows.is_empty() {
        tracing::info!("Backfilled {} property embeddings", rows.len());
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_serialize() {
        let p = Property {
            id: 1,
            name: Some("Test House".into()),
            image_url: Some("https://example.com/img.jpg".into()),
            region: Some("Melbourne".into()),
            bedroom_no: Some(3),
            price: Some(500_000),
        };
        let json = serde_json::to_value(&p).unwrap();
        assert_eq!(json["id"], 1);
        assert_eq!(json["name"], "Test House");
        assert_eq!(json["region"], "Melbourne");
        assert_eq!(json["bedroom_no"], 3);
        assert_eq!(json["price"], 500_000);
    }

    #[test]
    fn test_property_deserialize() {
        let json = r#"{"id":2,"name":"Beach Villa","image_url":null,"region":"Sydney","bedroom_no":2,"price":750000}"#;
        let p: Property = serde_json::from_str(json).unwrap();
        assert_eq!(p.id, 2);
        assert_eq!(p.name.as_deref(), Some("Beach Villa"));
        assert!(p.image_url.is_none());
    }

    #[test]
    fn test_property_clone() {
        let p = Property {
            id: 1,
            name: Some("Clone Test".into()),
            image_url: None,
            region: None,
            bedroom_no: None,
            price: None,
        };
        let p2 = p.clone();
        assert_eq!(p.id, p2.id);
        assert_eq!(p.name, p2.name);
    }
}
