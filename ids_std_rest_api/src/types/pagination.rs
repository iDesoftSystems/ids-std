use ids_std_domain::pagination::{Page, PaginationQuery};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page_size: u64,
    pub page: u64,
}

impl Into<PaginationQuery> for PaginationParams {
    fn into(self) -> PaginationQuery {
        PaginationQuery {
            page_size: self.page_size,
            page: self.page - 1,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Paged<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

impl<O> Paged<O> {
    pub fn from<I, F>(value: &Page<I>, mapper: F) -> Self
    where
        F: FnMut(&I) -> O,
    {
        Self {
            data: value.data.iter().map(mapper).collect(),
            total: value.total,
            page: value.page,
            page_size: value.page_size,
        }
    }
}

impl<I> Into<Page<I>> for Paged<I> {
    fn into(self) -> Page<I> {
        Page {
            data: self.data,
            total: self.total,
            page: self.page,
            page_size: self.page_size,
        }
    }
}

impl<I> Into<Paged<I>> for Page<I>
where
    I: Clone,
{
    fn into(self) -> Paged<I> {
        Paged {
            data: self.data.to_vec(),
            total: self.total,
            page: self.page,
            page_size: self.page_size,
        }
    }
}
