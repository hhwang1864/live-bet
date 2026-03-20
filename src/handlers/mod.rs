pub mod properties;
pub mod search;
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

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_success() {
        let mut tera = Tera::default();
        tera.add_raw_template("test.html", "Hello {{ name }}!")
            .unwrap();
        let tera = Arc::new(tera);
        let mut ctx = tera::Context::new();
        ctx.insert("name", "World");

        let resp = render(&tera, "test.html", ctx);
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[test]
    fn test_render_missing_template() {
        let tera = Arc::new(Tera::default());
        let ctx = tera::Context::new();

        let resp = render(&tera, "nonexistent.html", ctx);
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
