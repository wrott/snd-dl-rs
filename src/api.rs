use bytes::Bytes;
use reqwest::blocking::Response;

use crate::models::{DownloadStreamResponse, DownloadTrackResponse, TrackInfo};

const CLIENT_ID: &str = "a3e059563d7fd3372b49b37f00a00bcf";

pub fn resolve_track_info(url_to_resolve: String) -> reqwest::Result<TrackInfo> {
    let url: String = format!("https://api-v2.soundcloud.com/resolve?url={}&client_id={}", url_to_resolve, CLIENT_ID);
    reqwest::blocking::get(url)?.json()
}

pub fn get_download_link(id: u64) -> reqwest::Result<DownloadTrackResponse> {
    let url: String = format!("https://api-v2.soundcloud.com/tracks/{}/download?client_id={}", id, CLIENT_ID);
    let response: Response = reqwest::blocking::get(url)?;
    response.json()
}

pub fn get_hls_link(url: String) -> reqwest::Result<DownloadStreamResponse> {
    let auth_param: String = format!("?client_id={}", CLIENT_ID);
    let authenticated_url: String = format!("{}{}", url, auth_param);
    let response: Response = reqwest::blocking::get(authenticated_url)?;
    response.json()
}

pub fn get_bytes(url: String) -> reqwest::Result<Bytes> {
    let response: Response = reqwest::blocking::get(url)?;
    response.bytes()
}