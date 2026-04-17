#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::error::Error;

use axum::{Form, Router, extract::Query, http::HeaderMap, response::{IntoResponse, Redirect, Response}, routing::{get, post}};
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use maud::{Markup, html};
use reqwest::StatusCode;
use tokio::signal;
use tower_http::services::ServeDir;

use crate::objects::{AlbumRequest, ArtistRequest, MusicRequest, SearchParameters, SearchParametersFixed};

mod services;
mod views;
mod objects;
mod apis;
mod controllers;

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
        .route("/search", get(controllers::search::search_controller))
        .route("/request_music", post(controllers::request::request_music_controller))
        .route("/request_album", post(controllers::request::request_album_controller))
        .route("/request_artist", post(controllers::request::request_artist_controller))
        .nest_service("/static", ServeDir::new("./static/"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.map_err(Box::new)?;

    info!("Listenning on http://0.0.0.0:3000");
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


struct BadRequest(String);
impl IntoResponse for BadRequest {
    fn into_response(self) -> Response {
        error!("Error during request: {}", &self.0);
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

impl From<String> for BadRequest {
    fn from(value: String) -> Self {
        BadRequest(value)
    }
}