use axum::{
    Router,
    routing::{get, post},
};

use crate::ModulesExt;

pub mod auth;
pub mod profile;

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new()
        .route("/sign-up", post(auth::sign_up::<M>))
        .route("/sign-in", post(auth::sign_in::<M>))
        .route("/profile", get(profile::get_profile::<M>))
}
