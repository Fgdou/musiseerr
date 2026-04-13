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
    pub year: String,
    pub album: String,
    pub artist_id: String,
    pub album_id: String,
    pub id: String,
}
