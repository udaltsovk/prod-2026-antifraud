use application::{
    Application,
    usecase::fraud_rule::{
        CreateFraudRuleUsecase, ListFraudRulesUsecase,
        NormalizeDslExpressionUsecase,
    },
};
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
    dto::fraud_rule::{
        CreateFraudRuleDto, FraudRuleDslExpressionDto, FraudRuleDto,
        ValidatedFraudRuleDto,
    },
    errors::ApiResult,
    extractors::{Json, session::AdminSession},
};

pub mod by_id;

pub fn router<App>() -> Router<App>
where
    App: Application,
{
    Router::new()
        .route(
            "/",
            post(create_fraud_rule::<App>).get(list_fraud_rules::<App>),
        )
        .route(
            "/{fraud_rule_id}",
            get(by_id::get_fraud_rule_by_id::<App>)
                .put(by_id::update_fraud_rule_by_id::<App>)
                .delete(by_id::disable_fraud_rule_by_id::<App>),
        )
        .route("/validate", post(validate_fraud_rule::<App>))
}

pub async fn create_fraud_rule<App>(
    app: State<App>,
    AdminSession {
        ..
    }: AdminSession,
    Json(source): Json<CreateFraudRuleDto>,
) -> ApiResult<impl IntoResponse>
where
    App: CreateFraudRuleUsecase,
{
    let source = source.parse()?;

    app.create_fraud_rule(source)
        .await?
        .pipe(FraudRuleDto::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

pub async fn list_fraud_rules<App>(
    app: State<App>,
    AdminSession {
        ..
    }: AdminSession,
) -> ApiResult<impl IntoResponse>
where
    App: ListFraudRulesUsecase,
{
    app.list_fraud_rules(None)
        .await?
        .into_iter()
        .map(FraudRuleDto::from)
        .collect::<Vec<_>>()
        .pipe(Json)
        .pipe(Ok)
}

pub async fn validate_fraud_rule<App>(
    app: State<App>,
    AdminSession {
        ..
    }: AdminSession,
    Json(expression): Json<FraudRuleDslExpressionDto>,
) -> ApiResult<impl IntoResponse>
where
    App: NormalizeDslExpressionUsecase,
{
    let dsl_expression = expression.parse()?;

    app.normalize_dsl_expression(&dsl_expression)?
        .pipe(ValidatedFraudRuleDto::from)
        .pipe(Json)
        .pipe(Ok)
}
