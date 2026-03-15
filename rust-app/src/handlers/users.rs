use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use serde::Deserialize;
use tera::Context;

use crate::{models::user, AppState};

use super::render;

pub async fn new_form(State(state): State<AppState>) -> Response {
    render(&state.tera, "users/new.html", Context::new())
}

#[derive(Deserialize)]
pub struct UserForm {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub async fn create(
    State(state): State<AppState>,
    Form(form): Form<UserForm>,
) -> impl IntoResponse {
    user::create_user(
        &state.db,
        form.first_name.as_deref().unwrap_or(""),
        form.last_name.as_deref().unwrap_or(""),
        form.email.as_deref().unwrap_or(""),
        form.password.as_deref().unwrap_or(""),
    )
    .await;
    Redirect::to("/")
}
