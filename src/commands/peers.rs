use std::{
    net::{Ipv4Addr, SocketAddrV4},
    path::PathBuf,
};

use clap::Args;
use serde::Deserialize;
use serde_bytes::ByteBuf;

use crate::torrent::decode_torrent;

#[derive(Deserialize, Debug)]
struct TrackerResponse {
    peers: ByteBuf,
}

#[derive(Args, Debug)]
pub struct PeersArgs {
    pub torrent: PathBuf,
}

impl PeersArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        let contents = std::fs::read(&self.torrent)?;
        let torrent = decode_torrent(&contents);

        let url = format!(
            "{}?info_hash={}&peer_id=12312312303213213210&port=6881&uploaded=0&downloaded=0&left={}&compact=1",
            &torrent.announce,
            encode_bytes(&torrent.info_bytes()),
            torrent.info.length
        );

        let response = reqwest::blocking::get(url)?.bytes()?;

        let tracker: TrackerResponse =
            serde_bencode::from_bytes(&response).expect("Failed to parse response");

        let peers: Vec<String> = tracker
            .peers
            .chunks_exact(6)
            .map(|chunk| {
                SocketAddrV4::new(
                    Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]),
                    u16::from_be_bytes([chunk[4], chunk[5]]),
                )
                .to_string()
            })
            .collect();

        println!("{}", peers.join("\n"));

        Ok(())
    }
}

fn encode_bytes(t: &[u8; 20]) -> String {
    let mut encoded = String::with_capacity(3 * t.len());
    for &byte in t {
        encoded.push('%');
        encoded.push_str(&hex::encode(&[byte]));
    }
    encoded
}
