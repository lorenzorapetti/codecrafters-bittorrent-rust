use std::{
    io::{Read, Write},
    net::{SocketAddrV4, TcpStream},
    path::PathBuf,
};

use rand::RngExt;

use clap::Args;

use crate::torrent::decode_torrent;

#[derive(Args, Debug)]
pub struct HandshakeArgs {
    pub torrent: PathBuf,

    pub peer: SocketAddrV4,
}

impl HandshakeArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        let torrent = decode_torrent(&std::fs::read(&self.torrent)?);
        let mut stream = TcpStream::connect(&self.peer)?;

        let mut buf = [0u8; 68];
        buf[0] = 19;
        buf[1..20].copy_from_slice("BitTorrent protocol".as_bytes());
        buf[28..48].copy_from_slice(&torrent.info_bytes());
        buf[48..].copy_from_slice(&rand::rng().random::<[u8; 20]>());

        stream.write(&buf)?;

        buf = [0u8; 68];
        stream.read(&mut buf)?;

        println!("Peer ID: {}", hex::encode(&buf[48..]));

        Ok(())
    }
}
