use std::time::Duration;

use crate::apis::{self, lidarr::{AddOptions, ArtistRequest}};

pub async fn music(id: &str) -> Result<(), String> {
    let mbz_music = apis::musicbrainz::get_music(id).await;
    let mbz_album = mbz_music.releases.as_ref().unwrap().first().unwrap();
    let mbz_album_id = &mbz_album.release_group.id;
    let mbz_artist = &mbz_music.artist_credit.first().unwrap().artist;
    let mbz_artist_id = &mbz_artist.id;
    

    let lidarr_album = apis::lidarr::get_album(mbz_album_id).await;

    match lidarr_album {
        None => {
            request_artist(mbz_artist_id, &mbz_artist.name, Monitoring::None).await?;

            tokio::time::sleep(Duration::from_secs(5)).await;

            let lidarr_album = apis::lidarr::get_album(mbz_album_id).await.ok_or::<String>("Failed to get albums of new artist".into())?;

            request_album(lidarr_album.id).await?;
        },
        Some(lidarr_album) => {
            if lidarr_album.monitored {
                return Err("Music already monitored".into())
            } else {
                request_album(lidarr_album.id).await?
            }
        }
    }


    Ok(())
}

pub async fn artist(id: &str) -> Result<(), String> {
    let mbz_artist = apis::musicbrainz::get_artist(id).await;
    let lidarr_artist = apis::lidarr::get_artist(id).await;

    match lidarr_artist {
        Some(artist) => {
            let albums = apis::lidarr::get_albums_from_artist(artist.id).await;
            let ids: Vec<_> = albums.into_iter().map(|a| a.id).collect();

            apis::lidarr::monitor_albums(ids).await;

            Ok(())
        },
        None => {
            request_artist(id, &mbz_artist.name, Monitoring::All).await?;
            Ok(())
        }
    }
}

pub async fn album(id: &str) -> Result<(), String> {
    let mbz_album = apis::musicbrainz::get_album(id).await;
    let artist = &mbz_album.artist_credit.first().unwrap().artist;
    let lidarr_album = apis::lidarr::get_album(id).await;
    let lidarr_artist = apis::lidarr::get_artist(&artist.id).await;

    match (lidarr_artist, lidarr_album) {
        (None, _) => {
            request_artist(&artist.id, &artist.name, Monitoring::None).await?;
            tokio::time::sleep(Duration::from_secs(5)).await;
            let lidarr_album = apis::lidarr::get_album(&mbz_album.id).await.ok_or::<String>("Failed to get albums of new artist".into())?;

            request_album(lidarr_album.id).await?;

            Ok(())
        },
        (Some(_), None) => Err("Album not found in artist".into()),
        (Some(artist), Some(album)) => {
            request_album(album.id).await?;
            Ok(())
        }
    }
}

enum Monitoring {
    All,
    None,
}

async fn request_artist(musicbrainz_artist_id: &str, artist_name: &str, monitoring: Monitoring) -> Result<(), String> {
    let monitor = match monitoring {
        Monitoring::All => "all",
        Monitoring::None => "existing",
    };

    let defaults = apis::lidarr::get_defaults().await;
    let default = defaults.first().unwrap();

    let request = ArtistRequest {
        add_options: AddOptions {
            monitor: monitor.into(),
            search_for_missing_albums: false,
        },
        foreign_artist_id: musicbrainz_artist_id.to_string(),
        root_folder_path: default.path.clone(),
        path: format!("{}/{}", default.path, artist_name),
        quality_profile_id: default.default_quality_profile_id,
        metadata_profile_id: default.default_metadata_profile_id,
        artist_name: artist_name.into(),
        monitored: true
    };

    apis::lidarr::add_artist(request).await;

    Ok(())
}
async fn request_album(lidarr_album_id: u32) -> Result<(), String> {
    let _: () = apis::lidarr::monitor_albums(vec!(lidarr_album_id)).await;
    Ok(())
}