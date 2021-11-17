use std::io;
use std::io::Read;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DownloadTrackResponse {
    #[serde(rename = "redirectUri")]
    pub redirect_url: String
}

#[derive(Deserialize, Debug)]
pub struct DownloadStreamResponse {
    pub url: String
}

#[derive(Deserialize, Debug)]
pub struct TrackInfo {
    pub id: u64,
    pub downloadable: bool,
    pub permalink: String,
    pub purchase_url: Option<String>,
    media: Media
}

impl TrackInfo {
    pub fn get_stream_url(&self) -> Option<String> {
        self.media
            .transcodings
            .iter()
            .find(|t| t.format.mime_type == "audio/mpeg")
            .map(|t| t.url.to_string())
    }
}

#[derive(Deserialize, Debug)]
struct Media {
    transcodings: Vec<Transcoding>
}

#[derive(Deserialize, Debug)]
struct Transcoding {
    url: String,
    format: Format,
}

#[derive(Deserialize, Debug)]
struct Format {
    mime_type: String
}
