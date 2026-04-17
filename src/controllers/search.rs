use axum::{extract::Query, http::HeaderMap};
use maud::{Markup, html};

use crate::{BadRequest, objects::{SearchParameters, SearchParametersFixed}, services, views};

impl SearchParameters {
    pub fn get(&self) -> Option<SearchParametersFixed> {
        match self {
            SearchParameters{
                query: Some(s),
                max: m,
                page: p,
                search_type: Some(search_type),
            } => {
                Some(SearchParametersFixed {
                    query: s.clone(),
                    max: m.unwrap_or(50).min(100),
                    page: p.unwrap_or(0).min(500),
                    search_type: search_type.clone(),
                })
            },
            _ => None,
        }
    }
}

impl SearchParametersFixed {
    pub fn get_offset(&self) -> u32 {
        self.page*self.max
    }
}

pub async fn search_controller(search_params: Query<SearchParameters>, headers: HeaderMap) -> Result<Markup, BadRequest> {
    let htmx = headers.get("HX-Request").is_some();

    let params = search_params.get();

    let search_bar = views::search::search_bar(&params);

    let search = match params {
        Some(p) => Some(services::search::search(&p).await?),
        None => None,
    };

    if htmx && let Some(search) = search {
        return Ok(views::search::search_result(&search));
    } 
    
    let content = html!(
        div {
            (search_bar)

            div id="search-content" {
                @match search {
                    None => (html!{}),
                    Some(search) => (views::search::search_result(&search))
                }
            }
        }
    );

    Ok(views::header::template(content))
}