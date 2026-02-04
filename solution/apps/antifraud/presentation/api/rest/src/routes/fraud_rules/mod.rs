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
        response::ResponseExt as _, validation::parseable::Parseable as _,
    },
    tap::Pipe as _,
};

use crate::{
    ModulesExt,
    dto::fraud_rule::{
        CreateFraudRuleDto, FraudRuleDslExpressionDto, FraudRuleDto,
        ValidatedFraudRuleDto,
    },
    errors::ApiResult,
    extractors::{Json, session::AdminSession},
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

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn create_fraud_rule<M>(
    modules: State<M>,
    AdminSession {
        user_role: requester_role,
        ..
    }: AdminSession,
    Json(source): Json<CreateFraudRuleDto>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let source = source.parse().map_err(Into::into);

    modules
        .fraud_rule_usecase()
        .create(requester_role, source)
        .await
        .map(FraudRuleDto::from)
        .map(Json)?
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
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
        .map(FraudRuleDto::from)
        .collect::<Vec<_>>()
        .pipe(Json)
        .pipe(Ok)
}

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn validate_fraud_rule<M>(
    modules: State<M>,
    AdminSession {
        user_role: requester_role,
        ..
    }: AdminSession,
    Json(expression): Json<FraudRuleDslExpressionDto>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let dsl_expression = expression.parse().map_err(Into::into);

    modules
        .fraud_rule_usecase()
        .normalize_dsl_expression(requester_role, dsl_expression)
        .map(ValidatedFraudRuleDto::from)?
        .pipe(Json)
        .pipe(Ok)
}
