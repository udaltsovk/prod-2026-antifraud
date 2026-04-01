use application::Application;
use axum::{Router, routing::get};

pub mod by_id;

pub fn router<App>() -> Router<App>
where
    App: Application,
{
    Router::new().route(
        "/{user_id}/risk-profile",
        get(by_id::user_risk_profile_by_id::<App>),
    )
}
