use ids_std_domain::{spi};
use ids_std_domain::api::failure::InvalidField;
use crate::failure::ApiFailure;

impl From<spi::failure::SaveRepoFailure> for ApiFailure {
    fn from(err: spi::failure::SaveRepoFailure) -> Self {
        match err {
            spi::failure::SaveRepoFailure::Unknown(msg) => Self::Unknown(msg),
        }
    }
}

impl From<Vec<InvalidField>> for ApiFailure {
    fn from(value: Vec<InvalidField>) -> Self {
        Self::InvalidFields(value)
    }
}


impl From<spi::failure::SelectRepoFailure> for ApiFailure {
    fn from(failure: spi::failure::SelectRepoFailure) -> Self {
        match failure {
            spi::failure::SelectRepoFailure::Unknown(msg) => {
                Self::Unknown(msg)
            }
        }
    }
}


