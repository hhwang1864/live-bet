use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::Key;
use sqlx::PgPool;
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;

pub mod db;
pub mod handlers;
pub mod models;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub tera: Arc<Tera>,
    pub key: Key,
}

// Allows SignedCookieJar to extract the Key from AppState automatically.
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (copy .env.example to .env)");

    let db = db::create_pool(&database_url).await;

    let tera = Tera::new("templates/**/*.html").expect("Failed to parse Tera templates");

    let secret = std::env::var("SECRET_KEY")
        .unwrap_or_else(|_| "0".repeat(64));
    let mut secret_bytes = secret.into_bytes();
    while secret_bytes.len() < 64 {
        secret_bytes.push(0);
    }
    let key = Key::from(&secret_bytes);

    let state = AppState {
        db,
        tera: Arc::new(tera),
        key,
    };

    let app = Router::new()
        // Properties
        .route("/", get(handlers::properties::index))
        .route("/properties/new", get(handlers::properties::new_form))
        .route("/properties/", post(handlers::properties::create))
        .route(
            "/properties/:id",
            get(handlers::properties::edit_form).post(handlers::properties::update_or_delete),
        )
        .route("/house", get(handlers::properties::house))
        .route("/auction", get(handlers::properties::auction))
        .route("/one_bedroom", get(handlers::properties::one_bedroom))
        .route("/two_bedroom", get(handlers::properties::two_bedroom))
        .route("/three_bedroom", get(handlers::properties::three_bedroom))
        // Users
        .route("/users/new", get(handlers::users::new_form))
        .route("/users", post(handlers::users::create))
        // Sessions
        .route("/sessions/new", get(handlers::sessions::new_form))
        .route("/sessions", post(handlers::sessions::create_or_delete))
        // Static files
        .nest_service("/public", ServeDir::new("public"))
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
