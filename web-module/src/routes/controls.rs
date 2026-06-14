use std::sync::Arc;

use axum::{Router, extract::State, response::IntoResponse, routing::get};

use crate::AppState;

pub fn routes() -> Router<std::sync::Arc<crate::AppState>> {
    Router::new()
        .route("/controls", get(controls))
        .route("/controls/content", get(controls_partial))
}

async fn controls(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state.render("controls.html", &())
}

async fn controls_partial(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state.render("controls-content.html", &())
}
