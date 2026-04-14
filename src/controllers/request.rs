use std::time::Duration;

use chrono::Utc;

use crate::apis::{self, lidarr::{AddOptions, ArtistRequest}};

pub async fn music(id: &str) -> Result<(), String> {
    let mbz_music = apis::musicbrainz::get_music(id).await;
    let mbz_album = mbz_music.releases.first().unwrap();
    let mbz_album_id = &mbz_album.release_group.id;
    let mbz_artist = &mbz_music.artist_credits.first().unwrap().artist;
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

enum Monitoring {
    All,
    None,
}

async fn request_artist(musicbrainz_artist_id: &str, artist_name: &str, monitoring: Monitoring) -> Result<(), String> {
    let monitor = match monitoring {
        Monitoring::All => "all",
        Monitoring::None => "none",
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
    };

    apis::lidarr::add_artist(request).await;

    Ok(())
}
async fn request_album(lidarr_album_id: u32) -> Result<(), String> {
    let _: () = apis::lidarr::monitor_album(lidarr_album_id).await;
    Ok(())
}