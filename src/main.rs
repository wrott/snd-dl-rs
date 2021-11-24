use std::{env, io, thread};
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;

use bytes::Bytes;
use clap::{App, Arg, SubCommand};
use indicatif::{ProgressBar, ProgressStyle};

use crate::api::get_bytes;

mod models;
mod api;
mod m3u8_parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn download_original_track(id: u64) -> reqwest::Result<Bytes> {
    let download_response = api::get_download_link(id)?;
    api::get_bytes(download_response.redirect_url)
}

fn save_track_locally(bytes: Bytes, name: String) -> std::io::Result<()> {
    let mut buffer = File::create(format!("{}.mp3", name))?;
    buffer.write_all(&bytes[..])
}

fn download_hls_track(url: String, name: String) {
    let hls_url = api::get_hls_link(url).map(|r| r.url).unwrap();
    let file = File::create(format!("{}.mp3", name)).unwrap();
    m3u8_parser::download_hls_stream_to_file(hls_url, &file);
}



fn main() -> () {
    let url = env::args().nth(1).expect("NEED AN URL SONG");
    let track_info = api::resolve_track_info(url)
        .expect("Failed to resolve track info");


    if track_info.downloadable {
        let raw_track = download_original_track(track_info.id)
            .expect("Failed to download raw track");
            save_track_locally(raw_track, track_info.permalink)
            .expect("Failed to save raw track");
    } else {
        let stream_url = track_info.get_stream_url()
            .expect("No original download or mpeg stream available for track");
            download_hls_track(stream_url, track_info.permalink)
    }
}