use axum::{Json, Router, routing::get};
use serde_json::json;

use crate::ModulesExt;

pub mod auth;

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new()
        .route("/ping", get(async || Json(json!({"status": "ok"}))))
        .nest("/auth", auth::router())
}
