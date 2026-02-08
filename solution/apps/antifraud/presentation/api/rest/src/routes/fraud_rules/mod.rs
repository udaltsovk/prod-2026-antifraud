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

pub async fn create_fraud_rule<M>(
    modules: State<M>,
    AdminSession {
        ..
    }: AdminSession,
    Json(source): Json<CreateFraudRuleDto>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let source = source.parse()?;

    modules
        .fraud_rule_usecase()
        .create(source)
        .await?
        .pipe(FraudRuleDto::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

pub async fn list_fraud_rules<M>(
    modules: State<M>,
    AdminSession {
        ..
    }: AdminSession,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .fraud_rule_usecase()
        .list(None)
        .await?
        .into_iter()
        .map(FraudRuleDto::from)
        .collect::<Vec<_>>()
        .pipe(Json)
        .pipe(Ok)
}

pub async fn validate_fraud_rule<M>(
    modules: State<M>,
    AdminSession {
        ..
    }: AdminSession,
    Json(expression): Json<FraudRuleDslExpressionDto>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let dsl_expression = expression.parse()?;

    modules
        .fraud_rule_usecase()
        .normalize_dsl_expression(dsl_expression)?
        .pipe(ValidatedFraudRuleDto::from)
        .pipe(Json)
        .pipe(Ok)
}
