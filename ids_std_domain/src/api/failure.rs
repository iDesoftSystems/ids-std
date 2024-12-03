#[derive(Debug, PartialEq)]
pub struct InvalidField {
    pub field: String,
    pub error: String,
}

impl InvalidField {
    pub fn new(field: String, err: String) -> Self {
        Self { field, error: err }
    }
}

#[derive(Debug, PartialEq)]
pub enum CreateDomainFailure {
    Unknown(String),
    Conflict(String),
    InvalidFields(Vec<InvalidField>),
    InvalidField(InvalidField),
}

#[derive(Debug, PartialEq)]
pub enum UpdateDomainFailure {
    InvalidFields(Vec<InvalidField>),
    InvalidField(InvalidField),
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
    Unknown(String),
    NotFound(String),
}
