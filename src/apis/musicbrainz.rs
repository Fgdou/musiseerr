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

pub async fn search_music(query: &str, limit: u32) -> SearchMusicResponse {
    let url = format!("{}/recording?query={}&limit={}", API_URL, query, limit);

    dbg!(&url);

    let res = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    dbg!(&res);

    res
}

pub async fn search_artist(query: &str, limit: u32) -> SearchArtistResponse {
    let url = format!("{}/artist?query={}&limit={}", API_URL, query, limit);

    dbg!(&url);

    let res = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    dbg!(&res);

    res
}

pub async fn search_album(query: &str, limit: u32) -> SearchAlbumResponse {
    let url = format!("{}/release-group?query={}&limit={}", API_URL, query, limit);

    dbg!(&url);

    let res = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    dbg!(&res);
    let res = serde_json::from_str(&res).unwrap();

    dbg!(&res);

    res
}

pub async fn get_artist(id: &str) -> Artist {
    let url = format!("{}/artist/{}?inc=release-groups", API_URL, id);

    dbg!(&url);

    let res = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    dbg!(&res);

    res
}

pub async fn get_music(id: &str) -> Recording {
    let url = format!("{}/recording/{}?inc=release-groups+releases+artist-credits", API_URL, id);

    dbg!(&url);

    let res = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    dbg!(&res);

    res
}


pub async fn get_album(id: &str) -> ReleaseGroup {
    let url = format!("{}/release-group/{}?inc=artist-credits", API_URL, id);

    dbg!(&url);

    let res = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Musiseer/1.0.0 { fabigoardou@gmail.com }")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    dbg!(&res);

    res
}