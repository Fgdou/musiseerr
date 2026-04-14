use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum SearchType {
    #[serde(rename = "music")]
    Music,
    #[serde(rename = "album")]
    Album,
    #[serde(rename = "artist")]
    Artist,
}

#[derive(Deserialize)]
pub struct SearchParameters {
    pub query: Option<String>,
    pub max: Option<u32>,
    #[serde(rename = "type")]
    pub search_type: Option<SearchType>,
}

#[derive(Serialize)]
pub enum SearchResult {
    Musics(Vec<Music>),
}

#[derive(Serialize)]
pub struct Music {
    pub artist: String,
    pub title: String,
    pub album: String,
    pub artist_id: String,
    pub album_id: String,
    pub id: String,
    pub monitored: bool,
    pub album_type: String,
}

#[derive(Deserialize)]
pub struct MusicRequest {
    pub music_id: String,
}