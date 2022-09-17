use {
    crate::bs58_to_pubkey,
    solana_sdk::pubkey::Pubkey,
    anyhow::{
        anyhow,
        Result
    },
};

fn keys_len_valid(keys: &[&str], expected_len: u8) -> Result<()>{
    if keys.len() as u8 != expected_len {
        return Err(anyhow!("Invalid pubkey string length"));
    } 
    return Ok(());
}

trait Dex<T>{
    fn load_from_array(keys: &[&str]) -> Result<T>;
    /// flip source and destination accounts
    fn reverse_to_from(&self) -> Result<T>;
}

/// Solana, Saros, Orca-/v2
pub struct Default{
    pub parent_program: Pubkey,
    pub pool_program: Pubkey,
    pub pool_authority: Pubkey,
    pub user_authority: Pubkey,
    pub user_source: Pubkey,
    pub pool_source: Pubkey,
    pub pool_destination: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee: Pubkey,
    pub token_program: Pubkey,
}

impl Dex<Self> for Default{
    fn load_from_array(keys: &[&str]) -> Result<Self>{
        keys_len_valid(keys, 10)?;
        let key = keys.into_iter();
        return Ok(Self{
            parent_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_source: bs58_to_pubkey(key.next().unwrap())?,
            pool_source: bs58_to_pubkey(key.next().unwrap())?,
            pool_destination: bs58_to_pubkey(key.next().unwrap())?,
            pool_mint: bs58_to_pubkey(key.next().unwrap())?,
            pool_fee: bs58_to_pubkey(key.next().unwrap())?,
            token_program: bs58_to_pubkey(key.next().unwrap())?,
        });
    } 

    fn reverse_to_from(&self) -> Result<Self>{
        // --
    }
}

pub struct Saber{
    pub parent_program: Pubkey,
    pub pool_program: Pubkey,
    pub pool_authority: Pubkey,
    pub user_authority: Pubkey,
    pub user_source: Pubkey,
    pub pool_source: Pubkey,
    pub pool_destination: Pubkey,
    pub admin_destination: Pubkey,
    pub token_program: Pubkey,
}

pub struct Step{
    pub parent_program: Pubkey,
    pub pool_program: Pubkey,
    pub pool_authority: Pubkey,
    pub user_authority: Pubkey,
    pub user_source: Pubkey,
    pub pool_source: Pubkey,
    pub pool_destination: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee: Pubkey,
    pub refund_to: Pubkey,
    pub token_program: Pubkey,
}

pub struct Stepn{
    pub parent_program: Pubkey,
    pub pool_program: Pubkey,
    pub pool_authority: Pubkey,
    pub user_authority: Pubkey,
    pub user_source: Pubkey,
    pub pool_source: Pubkey,
    pub pool_destination: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee: Pubkey,
    pub refund_to: Pubkey,
    pub token_program: Pubkey,
}

pub struct Aldrin{
    pub parent_program: Pubkey,
    pub pool_program: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_base_token_vault: Pubkey,
    pub pool_quote_token_vault: Pubkey,
    pub user_authority: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_accoount: Pubkey,
    pub curve: Pubkey,
    pub pool_fee: Pubkey,
}

/*
 * Serum trade fees are paid in the base token's denomination
 * `market_order_payer_token_account` is our fee payer. Identical
 * to `market_coin_wallet` in the example tx we based this off.
 */
pub struct Serum{
    pub parent_program: Pubkey,
    pub market: Pubkey,
    pub market_open_orders: Pubkey,
    pub market_request_queue: Pubkey,
    pub market_event_queue: Pubkey,
    pub market_bids: Pubkey,
    pub market_asks: Pubkey,
    pub market_order_payer_token_account: Pubkey, //user owned (fee payer)
    pub market_coin_vault: Pubkey,
    pub market_pc_vault: Pubkey,
    pub market_vault_signer: Pubkey,
    pub market_coin_wallet: Pubkey, //user owned (source account)
    pub user_authority: Pubkey,
    pub user_pc_wallet: Pubkey,
    pub dex_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
