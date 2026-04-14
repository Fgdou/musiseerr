use axum::{Form, Router, extract::Query, response::Response, routing::{get, post}};
use dotenv::dotenv;
use maud::{Markup, html};

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
        .route("/request_music", post(request_music_controller));

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

async fn search_controller(search_params: Query<SearchParameters>) -> Markup {
    let search_bar = views::search::search_bar(&search_params);

    let search = match (&search_params.query, search_params.max) {
        (Some(query), Some(limit)) => Some(controllers::search::search(query, limit).await),
        (Some(query), None) => Some(controllers::search::search(query, 50).await),
        _ => None
    };

    html!(
        div {
            (search_bar)

            @match search {
                None => (html!{}),
                Some(search) => (views::search::search_result(&search))
            }
        }
    )
}