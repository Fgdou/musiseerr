use axum::Form;

use crate::{BadRequest, objects::{AlbumRequest, ArtistRequest, MusicRequest}, services};

pub async fn request_music_controller(req: Form<MusicRequest>) -> Result<String, BadRequest> {
    services::request::music(&req.music_id).await?;
    Ok("OK".into())
}

pub async fn request_artist_controller(req: Form<ArtistRequest>) -> Result<String, BadRequest> {
    services::request::artist(&req.artist_id).await?;
    Ok("OK".into())
}

pub async fn request_album_controller(req: Form<AlbumRequest>) -> Result<String, BadRequest> {
    services::request::album(&req.album_id).await?;
    Ok("OK".into())
}