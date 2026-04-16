use crate::{apis::{self, musicbrainz::{Recording, ReleaseGroup}}, objects::{Album, Artist, Music, SearchResult, SearchType}};

pub async fn search(query: &str, limit: u32, search_type: &SearchType) -> Result<SearchResult, String> {
    Ok(match search_type {
        SearchType::Music => SearchResult::Musics(search_musics(query, limit).await?),
        SearchType::Album => SearchResult::Albums(search_albums(query, limit).await?),
        SearchType::Artist => SearchResult::Artists(search_artists(query, limit).await?),
    })
}

async fn recording_exist_in_lidarr(recording: &Recording) -> Result<bool, String> {
    let release = if let Some(releases) = &recording.releases && let Some(release) = releases.first() {
        release
    } else {
        return Ok(false)
    };
    
    let album_id = &release.release_group.id;
    let album = apis::lidarr::get_album(album_id).await?;

    let album = if let Some(album) = album {
        album
    } else {
        return Ok(false)
    };

    if album.monitored {
        return Ok(true);
    }

    let futures = album.releases.iter()
        .map(async |release| apis::lidarr::get_tracks(release.id).await);
    
    let tracks = futures::future::try_join_all(futures).await?;

    Ok(tracks.into_iter().flatten().any(|track| track.foreign_recording_id == recording.id && track.has_file))
}

async fn artist_exist_in_lidarr(artist: &apis::musicbrainz::Artist) -> Result<bool, String> {
    let artist = apis::lidarr::get_artist(&artist.id).await?;
    
    match artist {
        None => Ok(false),
        Some(artist) if !artist.monitored => Ok(false),
        Some(artist) => {
            let albums = apis::lidarr::get_albums_from_artist(artist.id).await?;

            Ok(albums.into_iter().all(|a| a.monitored))
        },
    }
}

async fn album_exist_in_lidarr(album: &ReleaseGroup) -> Result<bool, String> {
    let album = apis::lidarr::get_album(&album.id).await?;

    Ok(match album {
        None => false,
        Some(album) => album.monitored
    })
}

fn is_live(album: &ReleaseGroup) -> bool {
    let secondary_types_contains_live = match album.secondary_types.as_ref() {
        None => false,
        Some(list) => list.contains(&"Live".into()),
    };
    let first_type_live = album.primary_type != Some("Album".into());

    !first_type_live && !secondary_types_contains_live
}

async fn search_musics(query: &str, limit: u32) -> Result<Vec<Music>, String> {
    let result = apis::musicbrainz::search_music(query, limit).await?;

    let futures = result.recordings
        .into_iter()
        .filter_map(|mut recording| {
            let releases = recording.releases?;

            let new_releases: Vec<_> = releases.into_iter().filter(|release| {
                is_live(&release.release_group)
            }).collect();

            let new_is_empty = new_releases.is_empty();

            recording.releases = Some(new_releases);

            if new_is_empty {
                None
            } else {
                Some(recording)
            }
        })
        .map(|recording| async {
            let artist = match &recording.artist_credit.first() {
                None => return Ok::<Option<Music>, String>(None),
                Some(artist) => &artist.artist
            };
            let exist = artist_exist_in_lidarr(artist).await? && recording_exist_in_lidarr(&recording).await?;
            let album = match recording.releases {
                None => return Ok(None),
                Some(releases) => match releases.into_iter().next() {
                    None => return Ok(None),
                    Some(r) => r.release_group
                }
            };
            Ok(Some(Music {
                title: recording.title,
                artist: artist.name.clone(),
                artist_id: artist.id.clone(),
                album_id: album.id,
                album: album.title,
                id: recording.id,
                monitored: exist,
                album_type: album.primary_type.unwrap_or_default(),
            }))
        });

    Ok(futures::future::try_join_all(futures).await?
        .into_iter()
        .filter_map(|e| e)
        .collect())
}
async fn search_artists(query: &str, limit: u32) -> Result<Vec<Artist>, String> {
    let result = apis::musicbrainz::search_artist(query, limit).await?;

    let futures = result.artists.into_iter()
        .map(|artist| async {
            let exist = artist_exist_in_lidarr(&artist).await?;
            Ok(Artist {
                name: artist.name,
                id: artist.id,
                monitored: exist,
            })
        });

    futures::future::try_join_all(futures).await
}
async fn search_albums(query: &str, limit: u32) -> Result<Vec<Album>, String> {
    let results = apis::musicbrainz::search_album(query, limit).await?;

    let futures = results.release_groups.into_iter()
        .map(|album| async {
            let exist = album_exist_in_lidarr(&album).await?;
            let artist = match &album.artist_credit {
                None => return Ok::<Option<Album>, String>(None),
                Some(artist) => match artist.first() {
                    None => return Ok(None),
                    Some(artist) => &artist.artist,
                }
            };
            Ok(Some(Album {
                id: album.id,
                monitored: exist,
                artist_id: artist.id.clone(),
                name: album.title,
                artist: artist.name.clone(),
                album_type: album.primary_type.unwrap_or_default(),
            }))
        });

    Ok(futures::future::try_join_all(futures).await?
        .into_iter()
        .filter_map(|e| e)
        .collect())
}