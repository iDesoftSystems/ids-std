use ids_std_domain::{
    pagination::{Page, PaginationQuery},
    spi::failure::SelectRepoFailure,
};
use sea_orm::{ConnectionTrait, Paginator, SelectorTrait};

use crate::replier::Replier;

pub async fn fetch_page<C, S, O, F>(
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
        .map_err(Replier::selector)?;

    let model_items: Vec<O> = paginator
        .fetch_page(query.page)
        .await
        .map_err(Replier::selector)?
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
