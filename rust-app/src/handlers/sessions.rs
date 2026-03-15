use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use axum_extra::extract::cookie::{Cookie, SignedCookieJar};
use serde::Deserialize;
use tera::Context;

use crate::{models::user, AppState};

use super::render;

pub async fn new_form(State(state): State<AppState>) -> Response {
    render(&state.tera, "sessions/new.html", Context::new())
}

#[derive(Deserialize)]
pub struct SessionForm {
    pub email: Option<String>,
    pub password: Option<String>,
    pub _method: Option<String>,
}

pub async fn create_or_delete(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Form(form): Form<SessionForm>,
) -> Response {
    // Logout via hidden _method=delete
    if form._method.as_deref() == Some("delete") {
        let jar = jar.remove(Cookie::from("user_id"));
        return (jar, Redirect::to("/")).into_response();
    }

    let email = form.email.as_deref().unwrap_or("");
    let password = form.password.as_deref().unwrap_or("");

    if let Some(u) = user::find_user_by_email(&state.db, email).await {
        let hash = u.password_digest.as_deref().unwrap_or("");
        if user::verify_password(hash, password).await {
            let mut cookie = Cookie::new("user_id", u.id.to_string());
            cookie.set_http_only(true);
            let jar = jar.add(cookie);
            return (jar, Redirect::to("/")).into_response();
        }
    }

    render(&state.tera, "sessions/new.html", Context::new())
}
