use crate::failure::ApiFailure;

pub type ApiResult<T> = Result<axum::Json<T>, ApiFailure>;
