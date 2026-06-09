use clap::Args;

use crate::bencode::decode_bencoded_value;

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub value: String,
}

impl DecodeArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("{}", decode_bencoded_value(&self.value)?.to_string());
        Ok(())
    }
}
