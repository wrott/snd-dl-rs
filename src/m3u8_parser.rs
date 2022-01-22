use std::fs::File;
use std::io::Write;

use bytes::Bytes;
use m3u8_rs::{parse_playlist_res, MediaPlaylist, Playlist};

use crate::{get_bytes, m3u8_parser};

pub fn download_hls_stream_to_file(url: String, output_file: &File) {
    let m3u8_bytes = get_bytes(url).expect("Failed to get bytes for playlist");
    let parsed_m3u8 = m3u8_parser::parse_playlist_res(&m3u8_bytes[..]);
    match parsed_m3u8 {
        Ok(Playlist::MediaPlaylist(playlist)) => download_segments_to_file(playlist, output_file),
        Ok(Playlist::MasterPlaylist(_)) => {
            panic!("m3u8 should be playlist of segments not playlist of playlists")
        }
        Err(_) => panic!("m3u8 parsing error"),
    }
}

fn download_segments_to_file(playlist: MediaPlaylist, output_file: &File) {
    playlist
        .segments
        .iter()
        .for_each(|seg| write_seg_to_file(download_seg(seg.uri.to_string()), output_file))
}

fn download_seg(url: String) -> Bytes {
    get_bytes(url).expect("Failed to get bytes for segment")
}

fn write_seg_to_file(bytes: Bytes, mut output_file: &File) {
    output_file
        .write_all(&bytes[..])
        .expect("Failed to write segment bytes to file")
}
