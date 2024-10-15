use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FailureReply {
    pub message: String,
    pub errors: Vec<ValidationFailure>,
}

impl From<String> for FailureReply {
    fn from(value: String) -> Self {
        Self {
            message: value,
            errors: Vec::new(),
        }
    }
}

impl From<Vec<ValidationFailure>> for FailureReply {
    fn from(value: Vec<ValidationFailure>) -> Self {
        Self {
            message: String::from("validation error"),
            errors: value,
        }
    }
}

impl From<validator::ValidationErrors> for FailureReply {
    fn from(value: validator::ValidationErrors) -> Self {
        let mut val_errors: Vec<ValidationFailure> = value
            .field_errors()
            .into_iter()
            .map(|error| ValidationFailure::new(error.0.to_string(), error.1[0].code.to_string()))
            .collect();

        val_errors.sort_by(|a, b| a.field.to_lowercase().cmp(&b.field.to_lowercase()));

        Self::from(val_errors)
    }
}

#[derive(Debug, Serialize)]
pub struct ValidationFailure {
    pub field: String,
    pub error: String,
}

impl ValidationFailure {
    pub fn new(field: String, err: String) -> Self {
        Self { field, error: err }
    }
}
