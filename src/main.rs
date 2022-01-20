use std::{env, thread};
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

use bytes::Bytes;
use indicatif::{ProgressBar, ProgressStyle};

use crate::api::get_bytes;

mod models;
mod api;
mod m3u8_parser;

//const VERSION: &str = env!("CARGO_PKG_VERSION");

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


fn main() {
    let mut downloaded = 0;
    let url = env::args().nth(1).expect("NEED AN URL SONG");
    let track_info = api::resolve_track_info(url)
        .expect("Failed to resolve track info");

    if track_info.downloadable {
        let raw_track = download_original_track(track_info.id)
            .expect("Failed to download raw track");
        save_track_locally(raw_track, track_info.permalink)
            .expect("Failed to save raw track");
        let size = 16;
        let pb = ProgressBar::new(size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] \
        [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            //.with_key("eta", |state| format!("{:.1}s", state.eta().as_secs_f64()))
            .progress_chars("#>-"));

        while downloaded < size {
            let new = min(downloaded + 223211, size);
            downloaded = new;
            pb.set_position(new);
            thread::sleep(Duration::from_millis(12));
        }
    } else {
        let stream_url = track_info.get_stream_url()
            .expect("No original download or mpeg stream available for track");
        download_hls_track(stream_url, track_info.permalink)
    }
}