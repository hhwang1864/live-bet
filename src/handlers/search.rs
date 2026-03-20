use axum::{
    extract::{Query, State},
    response::Response,
};
use axum_extra::extract::cookie::SignedCookieJar;
use serde::Deserialize;

use crate::{models::search, AppState};

use super::render;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

pub async fn search(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Query(params): Query<SearchQuery>,
) -> Response {
    let query = params.q.unwrap_or_default();
    let mut ctx = super::properties::base_ctx(&state, &jar).await;

    if query.trim().is_empty() {
        ctx.insert("query", "");
        ctx.insert("results", &Vec::<search::SearchResult>::new());
    } else {
        let results = search::search_properties(&state.db, &query, 20).await;
        ctx.insert("query", &query);
        ctx.insert("results", &results);
    }

    render(&state.tera, "properties/search.html", ctx)
}
