pub mod properties;
pub mod sessions;
pub mod users;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use axum_extra::extract::cookie::SignedCookieJar;
use std::sync::Arc;
use tera::Tera;

pub fn render(tera: &Arc<Tera>, template: &str, ctx: tera::Context) -> Response {
    match tera.render(template, &ctx) {
        Ok(html) => Html(html).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template error in {}: {}", template, e),
        )
            .into_response(),
    }
}

pub fn get_user_id(jar: &SignedCookieJar) -> Option<i32> {
    jar.get("user_id")
        .and_then(|c| c.value().parse::<i32>().ok())
}
