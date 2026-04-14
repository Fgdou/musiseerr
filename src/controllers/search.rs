use crate::{apis::{self, musicbrainz::{Recording, ReleaseGroup}}, objects::{Music, SearchResult}};

pub async fn search(query: &str, limit: u32) -> SearchResult {
    let musics = search_musics(query, limit).await;

    SearchResult {
        musics,
    }
}

async fn recording_exist_in_lidarr(recording: &Recording) -> bool {
    if recording.releases.is_none() {
        return false;
    }
    let album_id = &recording.releases.as_ref().unwrap().first().unwrap().release_group.id;
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

fn is_live(album: &ReleaseGroup) -> bool {
    let secondary_types_contains_live = match album.secondary_types.as_ref() {
        None => false,
        Some(list) => list.contains(&"Live".into()),
    };
    let first_type_live = album.primary_type != Some("Album".into());

    !first_type_live && !secondary_types_contains_live
}

async fn search_musics(query: &str, limit: u32) -> Vec<Music> {
    let result = apis::musicbrainz::search_music(query, limit).await;

    let futures = result.recordings
        .into_iter()
        .filter_map(|mut recording| {
            if recording.releases.is_none() {
                return None;
            }
            recording.releases = Some(recording.releases.unwrap().into_iter().filter(|release| {
                is_live(&release.release_group)
            }).collect());
            if recording.releases.as_ref().unwrap().is_empty() {
                None
            } else {
                Some(recording)
            }
        })
        .map(|recording| async {
            let exist = recording_exist_in_lidarr(&recording).await;
            let artist = recording.artist_credit.into_iter().next().unwrap().artist;
            let album = recording.releases.unwrap().into_iter().next().unwrap().release_group;
            Music {
                title: recording.title,
                artist: artist.name,
                artist_id: artist.id,
                album_id: album.id,
                album: album.title,
                id: recording.id,
                monitored: exist,
                album_type: album.primary_type.unwrap_or_default(),
            }
        });

    futures::future::join_all(futures).await
}