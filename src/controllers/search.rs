use crate::{apis::{self, musicbrainz::Recording}, objects::{Music, SearchResult}};

pub async fn search(query: &str, limit: u32) -> SearchResult {
    let musics = search_musics(query, limit).await;

    SearchResult {
        musics,
    }
}

async fn recording_exist_in_lidarr(recording: &Recording) -> bool {
    let album_id = &recording.releases.first().unwrap().release_group.id;
    let album = apis::lidarr::get_album(album_id).await;

    let album = if let Some(album) = album {
        album
    } else {
        return false
    };

    if album.monitored {
        return true;
    }

    let futures = album.releases.iter()
        .map(async |release| apis::lidarr::get_tracks(release.id).await);
    
    let tracks = futures::future::join_all(futures).await;

    tracks.into_iter().flatten().any(|track| track.foreign_recording_id == recording.id && track.has_file)
}

async fn search_musics(query: &str, limit: u32) -> Vec<Music> {
    let result = apis::musicbrainz::search_music(query, limit).await;

    let futures = result.recordings
        .into_iter()
        .map(|recording| async {
            let exist = recording_exist_in_lidarr(&recording).await;
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
                monitored: exist
            }
        });

    futures::future::join_all(futures).await
}