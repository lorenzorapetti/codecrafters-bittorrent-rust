use std::path::PathBuf;

use clap::Args;

use crate::torrent::decode_torrent;

#[derive(Args, Debug)]
pub struct InfoArgs {
    pub torrent: PathBuf,
}

impl InfoArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        let contents = std::fs::read(&self.torrent)?;
        let torrent = decode_torrent(&contents);
        println!("{}", torrent);
        Ok(())
    }
}
