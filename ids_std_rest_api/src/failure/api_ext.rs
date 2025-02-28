use ids_std_domain::api::failure::{CreateDomainFailure, FindManyFailure, FindOneFailure, UpdateDomainFailure};
use crate::failure::ApiFailure;

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
