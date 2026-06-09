use clap::{Parser, Subcommand};

use crate::commands::{decode::DecodeArgs, info::InfoArgs, peers::PeersArgs};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Decode(DecodeArgs),
    Info(InfoArgs),
    Peers(PeersArgs),
}
