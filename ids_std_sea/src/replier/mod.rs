use ids_std_domain::spi::failure::{SaveRepoFailure, SelectRepoFailure};
use sea_orm::DbErr;

pub struct Replier;

impl Replier {
    pub fn selector(err: DbErr) -> SelectRepoFailure {
        SelectRepoFailure::Unknown(err.to_string())
    }

    pub fn saver(err: DbErr) -> SaveRepoFailure {
        SaveRepoFailure::Unknown(err.to_string())
    }
}
