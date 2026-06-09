mod bencode;
mod cli;
mod commands;
mod torrent;

use clap::Parser;

use crate::cli::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Decode(args) => args.run()?,
        Command::Info(args) => args.run()?,
        Command::Peers(args) => args.run()?,
        Command::Handshake(args) => args.run()?,
    };

    Ok(())
}
