use std::env;

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Album {
    pub id: u32,
    pub title: String,
    #[serde(rename = "artistId")]
    pub artist_id: u32,
    #[serde(rename = "foreignAlbumId")]
    pub foreign_album_id: String,
    #[serde(rename = "releaseDate")]
    pub release_date: DateTime<Utc>,
    pub artist: Artist,
    pub releases: Vec<Release>,
    pub monitored: bool
}

#[derive(Deserialize)]
pub struct Artist {
    #[serde(rename = "artistName")]
    pub artist_name: String,
    #[serde(rename = "foreignArtistId")]
    pub foreign_artist_id: String,
    pub id: u32,
}

#[derive(Deserialize)]
pub struct Release {
    pub id: u32,
    #[serde(rename = "foreignReleaseId")]
    pub foreign_release_id: String,
}

#[derive(Deserialize)]
pub struct Track {
    #[serde(rename = "artistId")]
    pub artist_id: u32,
    #[serde(rename = "foreignTrackId")]
    pub foreign_track_id: String,
    #[serde(rename = "foreignRecordingId")]
    pub foreign_recording_id: String,
    #[serde(rename = "albumId")]
    pub album_id: u32,
    pub title: String,
    pub id: u32,
    #[serde(rename = "hasFile")]
    pub has_file: bool,
}

pub async fn get_album(musicbrainz_id: &str) -> Option<Album> {
    let env_url = env::var("LIDARR_URL").expect("LIDARR_URL env is not set");
    let token = env::var("LIDARR_API_KEY").expect("LIDARR_API_KEY env is not set");
    let url = format!("{}/api/v1/album?foreignAlbumId={}", env_url, musicbrainz_id);

    dbg!(&url);

    let res: Vec<_> = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .bearer_auth(token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    res.into_iter().next()
}

pub async fn get_tracks(album_release_id: u32) -> Vec<Track> {
    let env_url = env::var("LIDARR_URL").expect("LIDARR_URL env is not set");
    let token = env::var("LIDARR_API_KEY").expect("LIDARR_API_KEY env is not set");
    let url = format!("{}/api/v1/track?albumReleaseId={}", env_url, album_release_id);

    dbg!(&url);

     reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .bearer_auth(token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}