use crate::{apis::{self, musicbrainz::{Recording, ReleaseGroup}}, objects::{Album, Artist, Music, SearchResult, SearchType}};

pub async fn search(query: &str, limit: u32, search_type: &SearchType) -> SearchResult {
    match search_type {
        SearchType::Music => SearchResult::Musics(search_musics(query, limit).await),
        SearchType::Album => SearchResult::Albums(search_albums(query, limit).await),
        SearchType::Artist => SearchResult::Artists(search_artists(query, limit).await),
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

async fn artist_exist_in_lidarr(artist: &apis::musicbrainz::Artist) -> bool {
    let artist = apis::lidarr::get_artist(&artist.id).await;
    
    match artist {
        None => false,
        Some(artist) if !artist.monitored => false,
        Some(artist) => {
            let albums = apis::lidarr::get_albums_from_artist(artist.id).await;

            albums.into_iter().all(|a| a.monitored)
        },
    }
}

async fn album_exist_in_lidarr(album: &ReleaseGroup) -> bool {
    let album = apis::lidarr::get_album(&album.id).await;

    match album {
        None => false,
        Some(album) => album.monitored
    }
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
            recording.releases.as_ref()?;
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
            let artist = &recording.artist_credit.first().unwrap().artist;
            let exist = artist_exist_in_lidarr(artist).await && recording_exist_in_lidarr(&recording).await;
            let album = recording.releases.unwrap().into_iter().next().unwrap().release_group;
            Music {
                title: recording.title,
                artist: artist.name.clone(),
                artist_id: artist.id.clone(),
                album_id: album.id,
                album: album.title,
                id: recording.id,
                monitored: exist,
                album_type: album.primary_type.unwrap_or_default(),
            }
        });

    futures::future::join_all(futures).await
}
async fn search_artists(query: &str, limit: u32) -> Vec<Artist> {
    let result = apis::musicbrainz::search_artist(query, limit).await;

    let futures = result.artists.into_iter()
        .map(|artist| async {
            let exist = artist_exist_in_lidarr(&artist).await;
            Artist {
                name: artist.name,
                id: artist.id,
                monitored: exist,
            }
        });

    futures::future::join_all(futures).await
}
async fn search_albums(query: &str, limit: u32) -> Vec<Album> {
    let results = apis::musicbrainz::search_album(query, limit).await;

    let futures = results.release_groups.into_iter()
        .map(|album| async {
            let exist = album_exist_in_lidarr(&album).await;
            Album {
                id: album.id,
                monitored: exist,
                artist_id: album.artist_credit.as_ref().unwrap().first().unwrap().artist.id.clone(),
                name: album.title,
                artist: album.artist_credit.as_ref().unwrap().first().unwrap().artist.name.clone(),
                album_type: album.primary_type.unwrap_or_default(),
            }
        });

    futures::future::join_all(futures).await
}