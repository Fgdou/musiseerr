use serde::{self, Deserialize};

const API_URL: &str = "https://musicbrainz.org/ws/2";

#[derive(Deserialize)]
pub struct SearchMusicResponse {
    pub recordings: Vec<Recording>,
}
#[derive(Deserialize)]
pub struct Recording {
    pub id: String,
    pub title: String,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Vec<ArtistCredit>,
    #[serde(rename = "first-release-date")]
    pub first_release_date: String,
    pub releases: Vec<Release>,
}
#[derive(Deserialize)]
pub struct ArtistCredit {
    pub artist: Artist
}
#[derive(Deserialize)]
pub struct Artist {
    pub name: String,
    pub id: String,
}
#[derive(Deserialize)]
pub struct Release {
    #[serde(rename = "release-group")]
    pub release_group: ReleaseGroup,
}

#[derive(Deserialize)]
pub struct ReleaseGroup {
    pub title: String,
    pub id: String,
}

pub async fn search_music(query: &str, limit: u32) -> SearchMusicResponse {
    let url = format!("{}/recording?query={}&limit={}", API_URL, query, limit);

    dbg!(&url);

    reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}