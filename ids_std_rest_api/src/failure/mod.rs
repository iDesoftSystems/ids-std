use axum::http::StatusCode;
use ids_std_domain::api::failure::{
    CreateDomainFailure, FindManyFailure, FindOneFailure, InvalidField, UpdateDomainFailure,
};

use crate::{replier::Replier, types::failure::FailureReply};

#[derive(Debug)]
pub enum ApiFailure {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    Unknown(String),
    Conflict(String),
    InvalidFields(Vec<InvalidField>),
}

impl axum::response::IntoResponse for ApiFailure {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiFailure::InternalServerError(msg) => {
                Replier::render(StatusCode::INTERNAL_SERVER_ERROR, FailureReply::from(msg))
            }
            ApiFailure::NotFound(msg) => {
                Replier::render(StatusCode::NOT_FOUND, FailureReply::from(msg))
            }
            ApiFailure::Unknown(msg) => {
                Replier::render(StatusCode::INTERNAL_SERVER_ERROR, FailureReply::from(msg))
            }
            ApiFailure::Conflict(msg) => {
                Replier::render(StatusCode::CONFLICT, FailureReply::from(msg))
            }
            ApiFailure::InvalidFields(fields) => {
                Replier::render(StatusCode::BAD_REQUEST, FailureReply::from(fields))
            }
            ApiFailure::BadRequest(msg) => {
                Replier::render(StatusCode::BAD_REQUEST, FailureReply::from(msg))
            }
        }
    }
}

impl From<CreateDomainFailure> for ApiFailure {
    fn from(err: CreateDomainFailure) -> Self {
        match err {
            CreateDomainFailure::Unknown(msg) => ApiFailure::Unknown(msg),
            CreateDomainFailure::Conflict(msg) => ApiFailure::Conflict(msg),
            CreateDomainFailure::InvalidFields(fields) => ApiFailure::InvalidFields(fields),
            CreateDomainFailure::InvalidField(field) => ApiFailure::InvalidFields(vec![field]),
        }
    }
}

impl From<UpdateDomainFailure> for ApiFailure {
    fn from(err: UpdateDomainFailure) -> Self {
        match err {
            UpdateDomainFailure::InvalidFields(fields) => ApiFailure::InvalidFields(fields),
            UpdateDomainFailure::InvalidField(field) => ApiFailure::InvalidFields(vec![field]),
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
