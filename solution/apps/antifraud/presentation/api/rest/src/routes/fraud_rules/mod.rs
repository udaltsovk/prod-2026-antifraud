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
    extractors::{Json, session::AdminSession},
    models::fraud_rule::{
        CreateJsonFraudRule, JsonFraudRule, JsonFraudRuleDslExpression,
        ValidatedJsonFraudRule,
    },
};

pub mod by_id;

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
        .route("/validate", post(validate_fraud_rule::<M>))
}

pub async fn create_fraud_rule<M>(
    modules: State<M>,
    AdminSession {
        user_role: requester_role,
        ..
    }: AdminSession,
    Json(source): Json<CreateJsonFraudRule>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let source = source.parse();

    modules
        .fraud_rule_usecase()
        .create(requester_role, source)
        .await
        .map(JsonFraudRule::from)
        .map(Json)?
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

pub async fn list_fraud_rules<M>(
    modules: State<M>,
    AdminSession {
        user_role: requester_role,
        ..
    }: AdminSession,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .fraud_rule_usecase()
        .list(requester_role, None)
        .await?
        .into_iter()
        .map(JsonFraudRule::from)
        .collect::<Vec<_>>()
        .pipe(Json)
        .pipe(Ok)
}

pub async fn validate_fraud_rule<M>(
    modules: State<M>,
    AdminSession {
        user_role: requester_role,
        ..
    }: AdminSession,
    Json(expression): Json<JsonFraudRuleDslExpression>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let dsl_expression = expression.parse();

    modules
        .fraud_rule_usecase()
        .normalize_dsl_expression(requester_role, dsl_expression)
        .map(ValidatedJsonFraudRule::from)?
        .pipe(Json)
        .pipe(Ok)
}
