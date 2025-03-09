use ids_std_domain::{
    pagination::{Page, PaginationQuery},
    spi::failure::SelectRepoFailure,
};
use sea_orm::{ConnectionTrait, Paginator, SelectorTrait};

use crate::convert::into::IntoDomain;

pub async fn fetch_page<C, S>(
    paginator: &Paginator<'_, C, S>,
    query: &PaginationQuery,
) -> Result<Page<S>, SelectRepoFailure>
where
    S: SelectorTrait<Item = S>,
    C: ConnectionTrait,
{
    let page_info = paginator
        .num_items_and_pages()
        .await
        .map_err(|err| err.into_domain())?;

    let model_items: Vec<<S as SelectorTrait>::Item> = paginator
        .fetch_page(query.page)
        .await
        .map_err(|err| err.into_domain())?;

    Ok(Page {
        data: model_items,
        total: page_info.number_of_items,
        page: page_info.number_of_pages,
        page_size: query.page_size,
    })
}

pub async fn fetch_page_mapper<C, S, O, F>(
    paginator: &Paginator<'_, C, S>,
    query: &PaginationQuery,
    mapper: F,
) -> Result<Page<O>, SelectRepoFailure>
where
    S: SelectorTrait,
    C: ConnectionTrait,
    F: FnMut(&<S as SelectorTrait>::Item) -> O,
{
    let page_info = paginator
        .num_items_and_pages()
        .await
        .map_err(|err| err.into_domain())?;

    let model_items: Vec<O> = paginator
        .fetch_page(query.page)
        .await
        .map_err(|err| err.into_domain())?
        .iter()
        .map(mapper)
        .collect();

    Ok(Page {
        data: model_items,
        total: page_info.number_of_items,
        page: page_info.number_of_pages,
        page_size: query.page_size,
    })
}
