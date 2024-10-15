use axum::http::StatusCode;
use ids_std_domain::api::failure::{
    CreateDomainFailure, FindManyFailure, FindOneFailure, UpdateDomainFailure,
};

use crate::{replier::Replier, types::failure::FailureReply};

#[derive(Debug)]
pub enum ApiFailure {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    ValidationError(validator::ValidationErrors),
}

impl axum::response::IntoResponse for ApiFailure {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiFailure::BadRequest(msg) => {
                Replier::render(StatusCode::BAD_REQUEST, FailureReply::from(msg))
            }
            ApiFailure::InternalServerError(msg) => {
                Replier::render(StatusCode::INTERNAL_SERVER_ERROR, FailureReply::from(msg))
            }
            ApiFailure::NotFound(msg) => {
                Replier::render(StatusCode::NOT_FOUND, FailureReply::from(msg))
            }
            ApiFailure::InvalidData(msg) => {
                Replier::render(StatusCode::UNPROCESSABLE_ENTITY, FailureReply::from(msg))
            }
            ApiFailure::Unknown(msg) => {
                Replier::render(StatusCode::INTERNAL_SERVER_ERROR, FailureReply::from(msg))
            }
            ApiFailure::Conflict(msg) => {
                Replier::render(StatusCode::CONFLICT, FailureReply::from(msg))
            }
            ApiFailure::ValidationError(errs) => {
                Replier::render(StatusCode::UNPROCESSABLE_ENTITY, FailureReply::from(errs))
            }
        }
    }
}

impl From<CreateDomainFailure> for ApiFailure {
    fn from(err: CreateDomainFailure) -> Self {
        match err {
            CreateDomainFailure::Unknown(msg) => ApiFailure::Unknown(msg),
            CreateDomainFailure::Conflict(msg) => ApiFailure::Conflict(msg),
            CreateDomainFailure::ValidationError(errs) => ApiFailure::ValidationError(errs),
            CreateDomainFailure::InvalidData(msg) => ApiFailure::InvalidData(msg),
        }
    }
}

impl From<UpdateDomainFailure> for ApiFailure {
    fn from(err: UpdateDomainFailure) -> Self {
        match err {
            UpdateDomainFailure::ValidationError(msg) => ApiFailure::ValidationError(msg),
            UpdateDomainFailure::InvalidData(msg) => ApiFailure::InvalidData(msg),
            UpdateDomainFailure::Unknown(msg) => ApiFailure::Unknown(msg),
            UpdateDomainFailure::Conflict(msg) => ApiFailure::Conflict(msg),
        }
    }
}

impl From<FindOneFailure> for ApiFailure {
    fn from(err: FindOneFailure) -> Self {
        match err {
            FindOneFailure::Unknown(msg) => ApiFailure::Unknown(msg),
            FindOneFailure::NotFound(msg) => ApiFailure::NotFound(msg),
        }
    }
}

impl From<FindManyFailure> for ApiFailure {
    fn from(err: FindManyFailure) -> Self {
        match err {
            FindManyFailure::Unknown(msg) => ApiFailure::Unknown(msg),
        }
    }
}
