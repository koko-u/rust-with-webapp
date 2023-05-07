use std::error;

use axum::async_trait;
use axum::http::Request;
use axum::{extract, http, Json};
use serde::Deserialize;
use validator::Validate;

use crate::errors::ServerError;

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, B, T> extract::FromRequest<S, B> for ValidatedJson<T>
where
    T: for<'a> Deserialize<'a> + Validate,
    for<'b> B: axum::body::HttpBody + 'b,
    B::Error: error::Error + Send + Sync,
    B::Data: Send,
    B: Send,
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, ServerError);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let validated = match axum::Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => match value.validate() {
                Ok(()) => Ok(ValidatedJson(value)),
                Err(e) => {
                    return Err((http::StatusCode::BAD_REQUEST, e.into()));
                }
            },
            Err(e) => {
                return Err((http::StatusCode::BAD_REQUEST, e.into()));
            }
        }?;

        Ok(validated)
    }
}
