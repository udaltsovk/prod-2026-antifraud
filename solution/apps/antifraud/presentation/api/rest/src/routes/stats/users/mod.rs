use axum::{Router, routing::get};

use crate::ModulesExt;

pub mod by_id;

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new().route(
        "/{user_id}/risk-profile",
        get(by_id::user_risk_profile_by_id::<M>),
    )
}
