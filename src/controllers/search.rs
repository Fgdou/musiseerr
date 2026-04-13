use crate::{apis, objects::{Music, SearchResult}};

pub async fn search(query: &str, limit: u32) -> SearchResult {
    let musics = search_musics(query, limit).await;

    SearchResult {
        musics,
    }
}

async fn search_musics(query: &str, limit: u32) -> Vec<Music> {
    let result = apis::musicbrainz::search_music(query, limit).await;

    result.recordings
        .into_iter()
        .map(|recording| {
            let artist = recording.artist_credit.into_iter().next().unwrap().artist;
            let album = recording.releases.into_iter().next().unwrap().release_group;
            Music {
                title: recording.title,
                artist: artist.name,
                year: recording.first_release_date,
                artist_id: artist.id,
                album_id: album.id,
                album: album.title,
                id: recording.id,
            }
        })
        .collect()
}
