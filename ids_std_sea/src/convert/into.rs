use ids_std_domain::spi::failure::{SaveRepoFailure, SelectRepoFailure};

pub trait IntoDomain<T> : Sized {
    fn into_domain(self) -> T;
}

impl IntoDomain<SelectRepoFailure> for sea_orm::DbErr {
    fn into_domain(self) -> SelectRepoFailure {
        SelectRepoFailure::Unknown(self.to_string())
    }
}

impl IntoDomain<SaveRepoFailure> for sea_orm::DbErr {
    fn into_domain(self) -> SaveRepoFailure {
        SaveRepoFailure::Unknown(self.to_string())
    }
}