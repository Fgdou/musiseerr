use serde::{self, Deserialize};

const API_URL: &str = "https://musicbrainz.org/ws/2";

#[derive(Deserialize, Debug)]
pub struct SearchMusicResponse {
    pub recordings: Vec<Recording>,
}
#[derive(Deserialize, Debug)]
pub struct Recording {
    pub id: String,
    pub title: String,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Vec<ArtistCredit>,
    pub releases: Option<Vec<Release>>,
}
#[derive(Deserialize, Debug)]
pub struct ArtistCredit {
    pub artist: Artist
}
#[derive(Deserialize, Debug)]
pub struct Artist {
    pub name: String,
    pub id: String,
}
#[derive(Deserialize, Debug)]
pub struct Release {
    #[serde(rename = "release-group")]
    pub release_group: ReleaseGroup,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseGroup {
    pub title: String,
    pub id: String,
    #[serde(rename = "primary-type")]
    pub primary_type: Option<String>,
    #[serde(rename = "secondary-types")]
    pub secondary_types: Option<Vec<String>>,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Option<Vec<ArtistCredit>>,
}

#[derive(Deserialize, Debug)]
pub struct SearchArtistResponse {
    pub artists: Vec<Artist>
}

#[derive(Deserialize, Debug)]
pub struct SearchAlbumResponse {
    #[serde(rename = "release-groups")]
    pub release_groups: Vec<ReleaseGroup>,
}

pub async fn search_music(query: &str, limit: u32, offset: u32) -> Result<SearchMusicResponse, String> {
    let url = format!("{}/recording?query={}&limit={}&offset={}", API_URL, query, limit, offset);

    dbg!(&url);

    reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn search_artist(query: &str, limit: u32, offset: u32) -> Result<SearchArtistResponse, String> {
    let url = format!("{}/artist?query={}&limit={}&offset={}", API_URL, query, limit, offset);

    dbg!(&url);

    reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn search_album(query: &str, limit: u32, offset: u32) -> Result<SearchAlbumResponse, String> {
    let url = format!("{}/release-group?query={}&limit={}&offset={}", API_URL, query, limit, offset);

    dbg!(&url);

    reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_artist(id: &str) -> Result<Artist, String> {
    let url = format!("{}/artist/{}?inc=release-groups", API_URL, id);

    dbg!(&url);

    reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_music(id: &str) -> Result<Recording, String> {
    let url = format!("{}/recording/{}?inc=release-groups+releases+artist-credits", API_URL, id);

    dbg!(&url);

    reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}


pub async fn get_album(id: &str) -> Result<ReleaseGroup, String> {
    let url = format!("{}/release-group/{}?inc=artist-credits", API_URL, id);

    dbg!(&url);

    reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}