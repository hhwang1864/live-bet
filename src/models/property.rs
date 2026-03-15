use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Property {
    pub id: i32,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub region: Option<String>,
    pub bedroom_no: Option<i32>,
    pub price: Option<i32>,
}

pub async fn all_properties(db: &PgPool) -> Vec<Property> {
    sqlx::query_as::<_, Property>("SELECT * FROM properties ORDER BY id")
        .fetch_all(db)
        .await
        .unwrap_or_default()
}

pub async fn get_property(db: &PgPool, id: i32) -> Option<Property> {
    sqlx::query_as::<_, Property>("SELECT * FROM properties WHERE id = $1")
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
    sqlx::query(
        "INSERT INTO properties(name, image_url, region, bedroom_no, price) VALUES($1,$2,$3,$4,$5)",
    )
    .bind(name)
    .bind(image_url)
    .bind(region)
    .bind(bedroom_no)
    .bind(price)
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
    sqlx::query(
        "UPDATE properties SET name=$2, image_url=$3, region=$4, bedroom_no=$5, price=$6 WHERE id=$1",
    )
    .bind(id)
    .bind(name)
    .bind(image_url)
    .bind(region)
    .bind(bedroom_no)
    .bind(price)
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
    sqlx::query_as::<_, Property>(
        "SELECT * FROM properties WHERE bedroom_no = 1 ORDER BY id",
    )
    .fetch_all(db)
    .await
    .unwrap_or_default()
}

pub async fn two_bedroom(db: &PgPool) -> Vec<Property> {
    sqlx::query_as::<_, Property>(
        "SELECT * FROM properties WHERE bedroom_no = 2 ORDER BY id",
    )
    .fetch_all(db)
    .await
    .unwrap_or_default()
}

pub async fn three_bedroom(db: &PgPool) -> Vec<Property> {
    sqlx::query_as::<_, Property>(
        "SELECT * FROM properties WHERE bedroom_no >= 3 ORDER BY id",
    )
    .fetch_all(db)
    .await
    .unwrap_or_default()
}
