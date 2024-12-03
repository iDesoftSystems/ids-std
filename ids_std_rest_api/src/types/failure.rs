use ids_std_domain::api::failure::InvalidField;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FailureReply {
    pub message: String,
    pub errors: Vec<InvalidFieldFailure>,
}

impl From<String> for FailureReply {
    fn from(value: String) -> Self {
        Self {
            message: value,
            errors: Vec::new(),
        }
    }
}

impl From<Vec<InvalidField>> for FailureReply {
    fn from(value: Vec<InvalidField>) -> Self {
        Self {
            message: String::from("validation error"),
            errors: value
                .into_iter()
                .map(|failure| InvalidFieldFailure::new(failure.field, failure.error))
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InvalidFieldFailure {
    pub field: String,
    pub error: String,
}

impl InvalidFieldFailure {
    pub fn new(field: String, err: String) -> Self {
        Self { field, error: err }
    }
}
