use axum::{Form, Router, extract::Query, http::HeaderMap, response::Response, routing::{get, post}};
use dotenv::dotenv;
use maud::{Markup, html};
use tower_http::services::ServeDir;

use crate::objects::{MusicRequest, SearchParameters};

mod controllers;
mod views;
mod objects;
mod apis;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Hello, world!");

    let app = Router::new()
        .route("/health", get(async || {
            "OK"
        }))
        .route("/search", get(search_controller))
        .route("/request_music", post(request_music_controller))
        .nest_service("/static", ServeDir::new("./static/"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn request_music_controller(req: Form<MusicRequest>) -> Response {
    let res = controllers::request::music(&req.music_id).await;

    match res {
        Err(e) => Response::builder().status(400).body(e.into()).unwrap(),
        _ => Response::builder().status(200).body("OK".into()).unwrap()
    }
}

async fn search_controller(search_params: Query<SearchParameters>, headers: HeaderMap) -> Markup {
    let htmx = headers.get("HX-Request").is_some();

    let search_bar = views::search::search_bar(&search_params);

    let search = match (&search_params.query, search_params.max, &search_params.search_type) {
        (Some(query), Some(limit), Some(search_type)) => Some(controllers::search::search(query, limit, search_type).await),
        (Some(query), None, Some(search_type)) => Some(controllers::search::search(query, 50, search_type).await),
        _ => None
    };

    if htmx {
        return views::search::search_result(&search.unwrap())
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

    views::header::template(content)
}