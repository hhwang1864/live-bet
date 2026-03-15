use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    Form,
};
use axum_extra::extract::cookie::SignedCookieJar;
use serde::Deserialize;
use tera::Context;

use crate::{
    models::{property, user},
    AppState,
};

use super::{get_user_id, render};

async fn base_ctx(state: &AppState, jar: &SignedCookieJar) -> Context {
    let mut ctx = Context::new();
    if let Some(uid) = get_user_id(jar) {
        if let Some(u) = user::find_user_by_id(&state.db, uid).await {
            ctx.insert("current_user", &u);
            ctx.insert("logged_in", &true);
            return ctx;
        }
    }
    ctx.insert("logged_in", &false);
    ctx
}

pub async fn index(State(state): State<AppState>, jar: SignedCookieJar) -> Response {
    let properties = property::all_properties(&state.db).await;
    let mut ctx = base_ctx(&state, &jar).await;
    ctx.insert("properties", &properties);
    render(&state.tera, "properties/index.html", ctx)
}

pub async fn new_form(State(state): State<AppState>, jar: SignedCookieJar) -> Response {
    if get_user_id(&jar).is_none() {
        return Redirect::to("/").into_response();
    }
    let ctx = base_ctx(&state, &jar).await;
    render(&state.tera, "properties/new.html", ctx)
}

#[derive(Deserialize)]
pub struct PropertyForm {
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub region: Option<String>,
    pub bedroom_no: Option<String>,
    pub price: Option<String>,
    pub _method: Option<String>,
}

pub async fn create(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Form(form): Form<PropertyForm>,
) -> Response {
    if get_user_id(&jar).is_none() {
        return Redirect::to("/").into_response();
    }
    let bedroom_no: i32 = form.bedroom_no.as_deref().unwrap_or("0").parse().unwrap_or(0);
    let price: i32 = form.price.as_deref().unwrap_or("0").parse().unwrap_or(0);
    property::create_property(
        &state.db,
        form.name.as_deref().unwrap_or(""),
        form.image_url.as_deref().unwrap_or(""),
        form.region.as_deref().unwrap_or(""),
        bedroom_no,
        price,
    )
    .await;
    Redirect::to("/").into_response()
}

pub async fn edit_form(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Path(id): Path<i32>,
) -> Response {
    if get_user_id(&jar).is_none() {
        return Redirect::to("/").into_response();
    }
    let prop = property::get_property(&state.db, id).await;
    let mut ctx = base_ctx(&state, &jar).await;
    ctx.insert("property", &prop);
    render(&state.tera, "properties/edit.html", ctx)
}

pub async fn update_or_delete(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Path(id): Path<i32>,
    Form(form): Form<PropertyForm>,
) -> Response {
    if get_user_id(&jar).is_none() {
        return Redirect::to("/").into_response();
    }
    match form._method.as_deref() {
        Some("delete") => {
            property::delete_property(&state.db, id).await;
        }
        _ => {
            let bedroom_no: i32 = form.bedroom_no.as_deref().unwrap_or("0").parse().unwrap_or(0);
            let price: i32 = form.price.as_deref().unwrap_or("0").parse().unwrap_or(0);
            property::update_property(
                &state.db,
                id,
                form.name.as_deref().unwrap_or(""),
                form.image_url.as_deref().unwrap_or(""),
                form.region.as_deref().unwrap_or(""),
                bedroom_no,
                price,
            )
            .await;
        }
    }
    Redirect::to("/").into_response()
}

pub async fn house(State(state): State<AppState>, jar: SignedCookieJar) -> Response {
    let properties = property::all_properties(&state.db).await;
    let mut ctx = base_ctx(&state, &jar).await;
    ctx.insert("properties", &properties);
    render(&state.tera, "properties/house.html", ctx)
}

pub async fn auction(State(state): State<AppState>, jar: SignedCookieJar) -> Response {
    let properties = property::all_properties(&state.db).await;
    let mut ctx = base_ctx(&state, &jar).await;
    ctx.insert("properties", &properties);
    render(&state.tera, "properties/auction.html", ctx)
}

pub async fn one_bedroom(State(state): State<AppState>, jar: SignedCookieJar) -> Response {
    let properties = property::one_bedroom(&state.db).await;
    let mut ctx = base_ctx(&state, &jar).await;
    ctx.insert("properties", &properties);
    render(&state.tera, "properties/one_bedroom.html", ctx)
}

pub async fn two_bedroom(State(state): State<AppState>, jar: SignedCookieJar) -> Response {
    let properties = property::two_bedroom(&state.db).await;
    let mut ctx = base_ctx(&state, &jar).await;
    ctx.insert("properties", &properties);
    render(&state.tera, "properties/two_bedroom.html", ctx)
}

pub async fn three_bedroom(State(state): State<AppState>, jar: SignedCookieJar) -> Response {
    let properties = property::three_bedroom(&state.db).await;
    let mut ctx = base_ctx(&state, &jar).await;
    ctx.insert("properties", &properties);
    render(&state.tera, "properties/three_bedroom.html", ctx)
}
