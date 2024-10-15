use crate::api;
use crate::spi;

impl From<spi::failure::SaveRepoFailure> for api::failure::CreateDomainFailure {
    fn from(err: spi::failure::SaveRepoFailure) -> Self {
        match err {
            spi::failure::SaveRepoFailure::Unknown(msg) => Self::Unknown(msg),
        }
    }
}

impl From<validator::ValidationErrors> for api::failure::CreateDomainFailure {
    fn from(errs: validator::ValidationErrors) -> Self {
        Self::ValidationError(errs)
    }
}

impl From<spi::failure::SelectRepoFailure> for api::failure::CreateDomainFailure {
    fn from(failure: spi::failure::SelectRepoFailure) -> Self {
        match failure {
            spi::failure::SelectRepoFailure::Unknown(msg) => {
                api::failure::CreateDomainFailure::Unknown(msg)
            }
        }
    }
}
