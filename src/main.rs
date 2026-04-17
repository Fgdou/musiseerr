#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::error::Error;

use axum::{Form, Router, extract::Query, http::HeaderMap, response::{Redirect, Response}, routing::{get, post}};
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use maud::{Markup, html};
use tokio::signal;
use tower_http::services::ServeDir;

use crate::objects::{AlbumRequest, ArtistRequest, MusicRequest, SearchParameters, SearchParametersFixed};

mod controllers;
mod views;
mod objects;
mod apis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting MusiSeerr");
    info!("Checking environment variables");
    apis::lidarr::verify_connection().await?;

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

    info!("Listenning on 0.0.0.0:3000");
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

impl SearchParameters {
    fn get(&self) -> Option<SearchParametersFixed> {
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
    fn get_offset(&self) -> u32 {
        self.page*self.max
    }
}

async fn search_controller(search_params: Query<SearchParameters>, headers: HeaderMap) -> Result<Markup, String> {
    let htmx = headers.get("HX-Request").is_some();

    let params = search_params.get();

    let search_bar = views::search::search_bar(&params);

    let search = match params {
        Some(p) => Some(controllers::search::search(&p).await?),
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