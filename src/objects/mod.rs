use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SearchParameters {
    pub query: Option<String>,
    pub max: Option<u32>,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub musics: Vec<Music>,
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