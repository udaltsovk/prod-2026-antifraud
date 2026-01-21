use std::collections::HashMap;

use axum::{
    RequestPartsExt as _,
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use strum::EnumString;

use crate::ApiError;

#[derive(EnumString, Debug)]
#[strum(serialize_all = "lowercase")]
#[allow(
    dead_code,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason
)]
pub enum ApiVersion {
    V1,
}

impl<S> FromRequestParts<S> for ApiVersion
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> = parts.extract().await?;

        let version = params.get("api_version").ok_or_else(|| {
            ApiError::UnknownApiVerRejection(
                "missing version param".to_string(),
            )
        })?;

        version
            .parse()
            .map_err(|_| ApiError::UnknownApiVerRejection(version.clone()))
    }
}
