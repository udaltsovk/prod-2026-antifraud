use application::usecase::fraud_rule::FraudRuleUseCase as _;
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use lib::{
    presentation::api::rest::{
        model::Parseable as _, response::ResponseExt as _,
    },
    tap::Pipe as _,
};

use crate::{
    ModulesExt,
    errors::ApiResult,
    extractors::{Json, session::UserSession},
    models::fraud_rule::{CreateJsonFraudRule, JsonFraudRule},
};

pub mod by_id;
pub mod validate;

pub fn router<M>() -> Router<M>
where
    M: ModulesExt,
{
    Router::new()
        .route("/", post(create_fraud_rule::<M>).get(list_fraud_rules::<M>))
        .route(
            "/{fraud_rule_id}",
            get(by_id::get_fraud_rule_by_id::<M>)
                .put(by_id::update_fraud_rule_by_id::<M>)
                .delete(by_id::disable_fraud_rule_by_id::<M>),
        )
    // .route("/validate", post(validate::validate::<M>))
}

pub async fn create_fraud_rule<M>(
    modules: State<M>,
    user_session: UserSession,
    Json(source): Json<CreateJsonFraudRule>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let source = source.parse();

    modules
        .fraud_rule_usecase()
        .create(user_session.user_role, source)
        .await
        .map(JsonFraudRule::from)
        .map(Json)?
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

pub async fn list_fraud_rules<M>(
    modules: State<M>,
    user_session: UserSession,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .fraud_rule_usecase()
        .list(user_session.user_role)
        .await?
        .into_iter()
        .map(JsonFraudRule::from)
        .collect::<Vec<_>>()
        .pipe(Json)
        .pipe(Ok)
}
