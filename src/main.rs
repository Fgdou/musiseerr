#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::error::Error;

use axum::{Form, Router, extract::Query, http::HeaderMap, response::{Redirect, Response}, routing::{get, post}};
use dotenv::dotenv;
use maud::{Markup, html};
use tokio::signal;
use tower_http::services::ServeDir;

use crate::objects::{AlbumRequest, ArtistRequest, MusicRequest, SearchParameters};

mod controllers;
mod views;
mod objects;
mod apis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    println!("Hello, world!");

    let app = Router::new()
        .route("/health", get(async || {
            "OK"
        }))
        .route("/", get(|| async {Redirect::permanent("/search")}))
        .route("/search", get(search_controller))
        .route("/request_music", post(request_music_controller))
        .route("/request_album", post(request_album_controller))
        .route("/request_artist", post(request_artist_controller))
        .nest_service("/static", ServeDir::new("./static/"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.map_err(Box::new)?;
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.map_err(Box::new)?;

    Ok(())
}

#[allow(clippy::expect_used)]
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn request_music_controller(req: Form<MusicRequest>) -> Result<Response, String> {
    let res = controllers::request::music(&req.music_id).await;

    match res {
        Err(e) => Response::builder().status(400).body(e.into()).map_err(|e| e.to_string()),
        _ => Response::builder().status(200).body("OK".into()).map_err(|e| e.to_string())
    }
}

async fn request_artist_controller(req: Form<ArtistRequest>) -> Result<Response, String> {
    let res = controllers::request::artist(&req.artist_id).await;

    match res {
        Err(e) => Response::builder().status(400).body(e.into()).map_err(|e| e.to_string()),
        _ => Response::builder().status(200).body("OK".into()).map_err(|e| e.to_string())
    }
}

async fn request_album_controller(req: Form<AlbumRequest>) -> Result<Response, String> {
    let res = controllers::request::album(&req.album_id).await;

    match res {
        Err(e) => Response::builder().status(400).body(e.into()).map_err(|e| e.to_string()),
        _ => Response::builder().status(200).body("OK".into()).map_err(|e| e.to_string())
    }
}

async fn search_controller(search_params: Query<SearchParameters>, headers: HeaderMap) -> Result<Markup, String> {
    let htmx = headers.get("HX-Request").is_some();

    let search_bar = views::search::search_bar(&search_params);

    let search = match (&search_params.query, search_params.max, &search_params.search_type) {
        (Some(query), Some(limit), Some(search_type)) => Some(controllers::search::search(query, limit, search_type).await),
        (Some(query), None, Some(search_type)) => Some(controllers::search::search(query, 50, search_type).await),
        _ => None
    };

    if htmx && let Some(search) = search {
        return Ok(views::search::search_result(&search?));
    } 
    
    let content = html!(
        div {
            (search_bar)

            div id="search-content" {
                @match search {
                    None => (html!{}),
                    Some(search) => (views::search::search_result(&search?))
                }
            }
        }
    );

    Ok(views::header::template(content))
}