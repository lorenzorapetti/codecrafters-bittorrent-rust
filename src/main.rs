use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};

fn decode_bencoded_value(encoded_value: &str) -> anyhow::Result<serde_json::Value> {
    let value: serde_bencode::value::Value = serde_bencode::from_str(encoded_value)
        .unwrap_or_else(|_| panic!("Failed to decode bencoded value: {}", encoded_value));

    decode(value)
}

fn decode(value: serde_bencode::value::Value) -> anyhow::Result<serde_json::Value> {
    match value {
        serde_bencode::value::Value::Bytes(b) => {
            let string = String::from_utf8(b)?;
            Ok(serde_json::Value::String(string))
        }
        serde_bencode::value::Value::Int(i) => Ok(serde_json::Value::Number(i.into())),
        serde_bencode::value::Value::List(l) => Ok(serde_json::Value::Array(
            l.into_iter()
                .map(|item| decode(item))
                .collect::<anyhow::Result<Vec<serde_json::Value>>>()?,
        )),
        serde_bencode::value::Value::Dict(d) => Ok(serde_json::Value::Object(
            d.into_iter()
                .map(|(key, item)| {
                    let key = String::from_utf8(key)?;
                    let value = decode(item)?;
                    Ok((key, value))
                })
                .collect::<anyhow::Result<serde_json::Map<String, serde_json::Value>>>()?,
        )),
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct TorrentInfo {
    /// Size of the file in bytes, for single-file torrents
    length: u32,

    /// Suggested name to save the file / directory as
    name: String,

    /// Number of bytes in each piece
    #[serde(rename = "piece length")]
    piece_length: i64,

    /// Concatenated SHA-1 hashes of each piece
    pieces: ByteBuf,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Torrent {
    /// URL to a "tracker", which is a central server that keeps track of peers
    /// participating in the sharing of a torrent.
    announce: String,

    /// A dictionary containing information about the files in the torrent.
    /// This is the main part of the torrent file and contains details about the files being shared,
    /// such as their names, sizes, and how they are divided into pieces.
    info: TorrentInfo,
}

impl Torrent {
    /// Computes the info hash of the torrent, which is a SHA-1 hash of the bencoded "info" dictionary.
    /// The info hash is used to uniquely identify the torrent and is essential for peer-to-peer sharing.
    fn info_hash(&self) -> String {
        let info_bytes = serde_bencode::to_bytes(&self.info).expect("Failed to bencode info");
        let hash = Sha1::digest(&info_bytes);
        hex::encode(hash)
    }

    /// Encodes all the pieces into a readable SHA-1 hash array
    fn piece_hashes(&self) -> Vec<String> {
        self.info.pieces.chunks(20).map(hex::encode).collect()
    }
}

impl std::fmt::Display for Torrent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tracker URL: {}\nLength: {}\nInfo Hash: {}\nPiece Hashes:\n{}",
            self.announce,
            self.info.length,
            self.info_hash(),
            self.piece_hashes().join("\n")
        )
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Decode { value: String },
    Info { torrent: PathBuf },
}

fn decode_torrent(torrent: &[u8]) -> Torrent {
    serde_bencode::from_bytes(torrent).unwrap_or_else(|e| panic!("Can't decode torrent: {:?}", e))
}

// Usage: your_program.sh decode "<encoded_value>"
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Decode { value } => println!("{}", decode_bencoded_value(&value)?.to_string()),
        Commands::Info { torrent } => {
            let contents = std::fs::read(torrent)?;
            let torrent = decode_torrent(&contents);
            println!("{}", torrent);
        }
    }

    Ok(())
}
