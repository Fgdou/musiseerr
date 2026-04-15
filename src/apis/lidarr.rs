use std::env;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Artist {
    #[serde(rename = "artistName")]
    pub artist_name: String,
    #[serde(rename = "foreignArtistId")]
    pub foreign_artist_id: String,
    pub id: u32,
    pub monitored: bool,
}

#[derive(Deserialize, Debug)]
pub struct Release {
    pub id: u32,
    #[serde(rename = "foreignReleaseId")]
    pub foreign_release_id: String,
}

#[derive(Deserialize, Debug)]
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

    res.into_iter().next().inspect(|r| {
        dbg!(r);
    })
}

pub async fn get_albums_from_artist(lidarr_artist_id: u32) -> Vec<Album> {
    let env_url = env::var("LIDARR_URL").expect("LIDARR_URL env is not set");
    let token = env::var("LIDARR_API_KEY").expect("LIDARR_API_KEY env is not set");
    let url = format!("{}/api/v1/album?artistId={}", env_url, lidarr_artist_id);

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

pub async fn get_artist(musicbrainz_id: &str) -> Option<Artist> {
    let env_url = env::var("LIDARR_URL").expect("LIDARR_URL env is not set");
    let token = env::var("LIDARR_API_KEY").expect("LIDARR_API_KEY env is not set");
    let url = format!("{}/api/v1/artist?mbId={}", env_url, musicbrainz_id);

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

    res.into_iter().next().inspect(|r| {
        dbg!(r);
    })
}

pub async fn get_tracks(album_release_id: u32) -> Vec<Track> {
    let env_url = env::var("LIDARR_URL").expect("LIDARR_URL env is not set");
    let token = env::var("LIDARR_API_KEY").expect("LIDARR_API_KEY env is not set");
    let url = format!("{}/api/v1/track?albumReleaseId={}", env_url, album_release_id);

    dbg!(&url);

    let res = reqwest::Client::new()
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

    dbg!(&res);

    res
}

#[derive(Serialize, Debug)]
pub struct ArtistRequest {
    #[serde(rename = "addOptions")]
    pub add_options: AddOptions,
    #[serde(rename = "foreignArtistId")]
    pub foreign_artist_id: String,
    #[serde(rename = "qualityProfileId")]
    pub quality_profile_id: u32,
    #[serde(rename = "metadataProfileId")]
    pub metadata_profile_id: u32,
    pub path: String,
    #[serde(rename = "rootFolderPath")]
    pub root_folder_path: String,
    #[serde(rename = "artistName")]
    pub artist_name: String,
    pub(crate) monitored: bool,
}

#[derive(Serialize, Debug)]
pub struct AddOptions {
    pub monitor: String,
    #[serde(rename = "searchForMissingAlbums")]
    pub search_for_missing_albums: bool,
}

pub async fn add_artist(artist: ArtistRequest) {
    let env_url = env::var("LIDARR_URL").expect("LIDARR_URL env is not set");
    let token = env::var("LIDARR_API_KEY").expect("LIDARR_API_KEY env is not set");
    let url = format!("{}/api/v1/artist", env_url);

    dbg!(&url, &artist);

    let res = reqwest::Client::new()
        .post(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .header("Content-Type", "application/json")
        .json(&artist)
        .bearer_auth(token)
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        panic!("Failed: {}", res.text().await.unwrap())
    }
}

#[derive(Serialize, Debug)]
struct AlbumMonitorRequest {
    #[serde(rename = "albumIds")]
    album_ids: Vec<u32>,
    monitored: bool,
}

pub async fn monitor_albums(lidarr_album_id: Vec<u32>) {
    let env_url = env::var("LIDARR_URL").expect("LIDARR_URL env is not set");
    let token = env::var("LIDARR_API_KEY").expect("LIDARR_API_KEY env is not set");
    let url = format!("{}/api/v1/album/monitor", env_url);

    let request = AlbumMonitorRequest {
        album_ids: lidarr_album_id,
        monitored: true,
    };

    dbg!(&url, &request);

    let res = reqwest::Client::new()
        .put(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .json(&request)
        .send()
        .await
        .unwrap();

    if !res.status().is_success() {
        panic!("Failed: {}", res.text().await.unwrap())
    }
}

#[derive(Deserialize)]
pub struct DefaultsResponse {
    pub id: u32,
    pub path: String,
    #[serde(rename = "defaultMetadataProfileId")]
    pub default_metadata_profile_id: u32,
    #[serde(rename = "defaultQualityProfileId")]
    pub default_quality_profile_id: u32,
}

pub async fn get_defaults() -> Vec<DefaultsResponse> {
    let env_url = env::var("LIDARR_URL").expect("LIDARR_URL env is not set");
    let token = env::var("LIDARR_API_KEY").expect("LIDARR_API_KEY env is not set");
    let url = format!("{}/api/v1/rootfolder", env_url);

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