use application::usecase::fraud_rule::{
    DisableFraudRuleByIdUsecase, GetFraudRuleByIdUsecase,
    UpdateFraudRuleByIdUsecase,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::{
    presentation::api::rest::{
        response::ResponseExt as _, validation::parseable::Parseable as _,
    },
    tap::Pipe as _,
    uuid::Uuid,
};

use crate::{
    dto::fraud_rule::{FraudRuleDto, FraudRuleUpdateDto},
    errors::ApiResult,
    extractors::{Json, Path, session::AdminSession},
};

pub async fn get_fraud_rule_by_id<App>(
    app: State<App>,
    AdminSession {
        ..
    }: AdminSession,
    Path(((), fraud_rule_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    App: GetFraudRuleByIdUsecase,
{
    app.get_fraud_rule_by_id(fraud_rule_id.into())
        .await?
        .pipe(FraudRuleDto::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

pub async fn update_fraud_rule_by_id<App>(
    app: State<App>,
    AdminSession {
        ..
    }: AdminSession,
    Path(((), fraud_rule_id)): Path<((), Uuid)>,
    Json(update): Json<FraudRuleUpdateDto>,
) -> ApiResult<impl IntoResponse>
where
    App: UpdateFraudRuleByIdUsecase,
{
    let update_result = update.parse()?;

    app.update_fraud_rule_by_id(fraud_rule_id.into(), update_result)
        .await?
        .pipe(FraudRuleDto::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

pub async fn disable_fraud_rule_by_id<App>(
    app: State<App>,
    AdminSession {
        ..
    }: AdminSession,
    Path(((), fraud_rule_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    App: DisableFraudRuleByIdUsecase,
{
    app.disable_fraud_rule_by_id(fraud_rule_id.into()).await?;

    StatusCode::NO_CONTENT.pipe(Ok)
}
