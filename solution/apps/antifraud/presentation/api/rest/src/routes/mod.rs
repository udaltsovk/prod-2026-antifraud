use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;

use crate::ModulesExt;

pub mod auth;
pub mod fraud_rules;
pub mod stats;
pub mod transactions;
pub mod users;

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new()
        .route("/ping", get(ping))
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/fraud-rules", fraud_rules::router())
        .nest("/transactions", transactions::router())
        .nest("/stats", stats::router())
}

pub async fn ping() -> impl IntoResponse {
    Json(json!({"status": "ok"}))
}
