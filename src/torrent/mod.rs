use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentInfo {
    /// Size of the file in bytes, for single-file torrents
    pub length: u32,

    /// Suggested name to save the file / directory as
    pub name: String,

    /// Number of bytes in each piece
    #[serde(rename = "piece length")]
    pub piece_length: i64,

    /// Concatenated SHA-1 hashes of each piece
    pub pieces: ByteBuf,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Torrent {
    /// URL to a "tracker", which is a central server that keeps track of peers
    /// participating in the sharing of a torrent.
    pub announce: String,

    /// A dictionary containing information about the files in the torrent.
    /// This is the main part of the torrent file and contains details about the files being shared,
    /// such as their names, sizes, and how they are divided into pieces.
    pub info: TorrentInfo,
}

impl Torrent {
    /// Returns the raw bytes of the "info" dictionary, which is used to compute the info hash.
    pub fn info_bytes(&self) -> [u8; 20] {
        let info_bytes = serde_bencode::to_bytes(&self.info).expect("Failed to bencode info");
        Sha1::digest(&info_bytes).into()
    }

    /// Computes the info hash of the torrent, which is a SHA-1 hash of the bencoded "info" dictionary.
    /// The info hash is used to uniquely identify the torrent and is essential for peer-to-peer sharing.
    pub fn info_hash(&self) -> String {
        hex::encode(self.info_bytes())
    }

    /// Encodes all the pieces into a readable SHA-1 hash array
    pub fn piece_hashes(&self) -> Vec<String> {
        self.info.pieces.chunks(20).map(hex::encode).collect()
    }
}

impl std::fmt::Display for Torrent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tracker URL: {}\nLength: {}\nInfo Hash: {}\nPiece Length: {}\nPiece Hashes:\n{}",
            self.announce,
            self.info.length,
            self.info_hash(),
            self.info.piece_length,
            self.piece_hashes().join("\n")
        )
    }
}

pub fn decode_torrent(torrent: &[u8]) -> Torrent {
    serde_bencode::from_bytes(torrent).unwrap_or_else(|e| panic!("Can't decode torrent: {:?}", e))
}
