use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Eq)]
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
    Artists(Vec<Artist>),
    Albums(Vec<Album>),
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

#[derive(Serialize)]
pub struct Artist {
    pub name: String,
    pub id: String,
    pub monitored: bool,
}

#[derive(Serialize)]
pub struct Album {
    pub name: String,
    pub id: String,
    pub monitored: bool,
    pub artist: String,
    pub artist_id: String,
    pub album_type: String,
}

#[derive(Deserialize)]
pub struct MusicRequest {
    pub music_id: String,
}

#[derive(Deserialize)]
pub struct ArtistRequest {
    pub artist_id: String,
}
#[derive(Deserialize)]
pub struct AlbumRequest {
    pub album_id: String,
}