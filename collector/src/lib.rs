pub mod dex;
use {
    bs58,
    anyhow::{
        Result,
        anyhow,
    },
    solana_sdk::pubkey::Pubkey,
};

pub fn bs58_to_pubkey(key: &str) -> Result<Pubkey>{
    if key.len() < 32 || key.len() > 35 {
        return Err(anyhow!("'{}' pubkey length invalid", key));
    }
    return Ok(Pubkey::new(&bs58::decode(key).into_vec()?));
}

