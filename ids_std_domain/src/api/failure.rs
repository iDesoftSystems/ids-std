use validator::ValidationErrors;

#[derive(Debug, PartialEq)]
pub enum CreateDomainFailure {
    ValidationError(ValidationErrors),
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}

#[derive(Debug, PartialEq)]
pub enum UpdateDomainFailure {
    ValidationError(ValidationErrors),
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}

#[derive(Debug, PartialEq)]
pub enum FindOneFailure {
    Unknown(String),
    NotFound(String),
}

#[derive(Debug, PartialEq)]
pub enum FindManyFailure {
    Unknown(String),
}

#[derive(Debug, PartialEq)]
pub enum DeleteOneFailure {
    InvalidData(String),
    Unknown(String),
    NotFound(String),
}
