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
    fn load_from_arr(keys: &[&str]) -> Result<T>;
    
    fn reverse_to_from(&self) -> T;
}

/// Solana, Saros, Orca, OrcV2
pub struct Default{
    pub parent_program: Pubkey,
    pub pool_program: Pubkey,
    pub pool_authority: Pubkey,
    pub user_authority: Pubkey,
    pub user_source: Pubkey,
    pub pool_source: Pubkey,
    pub pool_destination: Pubkey,
    pub user_destination: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee: Pubkey,
    pub token_program: Pubkey,
}

impl Dex<Self> for Default{
    fn load_from_arr(keys: &[&str]) -> Result<Self>{
        keys_len_valid(keys, 11)?;
        let mut key = keys.iter();
        return Ok(Self{
            parent_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_source: bs58_to_pubkey(key.next().unwrap())?,
            pool_source: bs58_to_pubkey(key.next().unwrap())?,
            pool_destination: bs58_to_pubkey(key.next().unwrap())?,
            user_destination: bs58_to_pubkey(key.next().unwrap())?,
            pool_mint: bs58_to_pubkey(key.next().unwrap())?,
            pool_fee: bs58_to_pubkey(key.next().unwrap())?,
            token_program: bs58_to_pubkey(key.next().unwrap())?,
        });
    } 

    fn reverse_to_from(&self) -> Self{
        return Self {
            parent_program: self.parent_program.clone(),
            pool_program: self.pool_program.clone(),
            pool_authority: self.pool_authority.clone(),
            user_authority: self.user_authority.clone(),
            user_source: self.user_destination.clone(),
            pool_source: self.pool_destination.clone(),
            pool_destination: self.pool_source.clone(),
            user_destination: self.user_source.clone(),
            pool_mint: self.pool_mint.clone(),
            pool_fee : self.pool_fee.clone(),
            token_program: self.token_program.clone(),
        }; 
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
    pub user_destination: Pubkey,
    pub admin_destination: Pubkey,
    pub token_program: Pubkey,
}

impl Dex<Saber> for Saber{
    fn load_from_arr(keys: &[&str]) -> Result<Self>{
        keys_len_valid(keys, 10)?;
        let mut key = keys.iter();
        return Ok(Self{
            parent_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_source: bs58_to_pubkey(key.next().unwrap())?,
            pool_source: bs58_to_pubkey(key.next().unwrap())?,
            pool_destination: bs58_to_pubkey(key.next().unwrap())?,
            user_destination: bs58_to_pubkey(key.next().unwrap())?,
            admin_destination: bs58_to_pubkey(key.next().unwrap())?,
            token_program: bs58_to_pubkey(key.next().unwrap())?,
        });
    }
    
    fn reverse_to_from(&self) -> Self{
        return Self{
            parent_program: self.parent_program.clone(),
            pool_program: self.pool_program.clone(),
            pool_authority: self.pool_authority.clone(),
            user_authority: self.user_authority.clone(),
            user_source: self.user_destination.clone(),
            pool_source: self.pool_destination.clone(),
            pool_destination: self.pool_source.clone(),
            user_destination: self.user_source.clone(),
            admin_destination: self.admin_destination.clone(),
            token_program: self.token_program.clone(),
        };
    }
}

pub struct Step{
    pub parent_program: Pubkey,
    pub pool_program: Pubkey,
    pub pool_authority: Pubkey,
    pub user_authority: Pubkey,
    pub user_source: Pubkey,
    pub pool_source: Pubkey,
    pub pool_destination: Pubkey,
    pub user_destination: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee: Pubkey,
    pub refund_to: Pubkey,
    pub token_program: Pubkey,
}

impl Dex<Step> for Step {
    fn load_from_arr(keys: &[&str]) -> Result<Self>{
        keys_len_valid(keys, 12)?;
        let mut key = keys.iter();
        return Ok(Self{
            parent_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_source: bs58_to_pubkey(key.next().unwrap())?,
            pool_source: bs58_to_pubkey(key.next().unwrap())?,
            pool_destination: bs58_to_pubkey(key.next().unwrap())?,
            user_destination: bs58_to_pubkey(key.next().unwrap())?,
            pool_mint: bs58_to_pubkey(key.next().unwrap())?,
            pool_fee: bs58_to_pubkey(key.next().unwrap())?,
            refund_to: bs58_to_pubkey(key.next().unwrap())?,
            token_program: bs58_to_pubkey(key.next().unwrap())?,
        });
    }
    
    fn reverse_to_from(&self) -> Self{
        return Self{
            parent_program: self.parent_program.clone(),
            pool_program: self.pool_program.clone(),
            pool_authority: self.pool_authority.clone(),
            user_authority: self.user_authority.clone(),
            user_source: self.user_destination.clone(),
            pool_source: self.pool_destination.clone(),
            pool_destination: self.pool_source.clone(),
            user_destination: self.user_source.clone(),
            pool_mint: self.pool_mint.clone(),
            pool_fee : self.pool_fee.clone(),
            refund_to: self.refund_to.clone(),
            token_program: self.token_program.clone(),
        };
    }
}

pub struct Stepn{
    pub parent_program: Pubkey,
    pub pool_program: Pubkey,
    pub pool_authority: Pubkey,
    pub user_authority: Pubkey,
    pub user_source: Pubkey,
    pub pool_source: Pubkey,
    pub pool_destination: Pubkey,
    pub user_destination: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee: Pubkey,
    pub refund_to: Pubkey,
}

impl Dex<Stepn> for Stepn{
    fn load_from_arr(keys: &[&str]) -> Result<Self>{
        keys_len_valid(keys, 11)?;
        let mut key = keys.iter();
        return Ok(Self{
            parent_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_source: bs58_to_pubkey(key.next().unwrap())?,
            pool_source: bs58_to_pubkey(key.next().unwrap())?,
            pool_destination: bs58_to_pubkey(key.next().unwrap())?,
            user_destination: bs58_to_pubkey(key.next().unwrap())?,
            pool_mint: bs58_to_pubkey(key.next().unwrap())?,
            pool_fee: bs58_to_pubkey(key.next().unwrap())?,
            refund_to: bs58_to_pubkey(key.next().unwrap())?,
        });
    }
    
    fn reverse_to_from(&self) -> Self{
        return Self{
            parent_program: self.parent_program.clone(),
            pool_program: self.pool_program.clone(),
            pool_authority: self.pool_authority.clone(),
            user_authority: self.user_authority.clone(),
            user_source: self.user_destination.clone(),
            pool_source: self.pool_destination.clone(),
            pool_destination: self.pool_source.clone(),
            user_destination: self.user_source.clone(),
            pool_mint: self.pool_mint.clone(),
            pool_fee : self.pool_fee.clone(),
            refund_to: self.refund_to.clone(),
        };
    }
}

trait Clob<T>{
    fn load_from_arr(keys: &[&str]) -> Result<T>;
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

impl Clob<Aldrin> for Aldrin{
    fn load_from_arr(keys: &[&str]) -> Result<Self>{
        keys_len_valid(keys, 11)?;
        let mut key = keys.iter();
        
        return Ok(Self{
            parent_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_program: bs58_to_pubkey(key.next().unwrap())?,
            pool_authority: bs58_to_pubkey(key.next().unwrap())?,
            pool_mint: bs58_to_pubkey(key.next().unwrap())?,
            pool_base_token_vault: bs58_to_pubkey(key.next().unwrap())?,
            pool_quote_token_vault: bs58_to_pubkey(key.next().unwrap())?,
            user_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_base_token_account: bs58_to_pubkey(key.next().unwrap())?,
            user_quote_token_accoount: bs58_to_pubkey(key.next().unwrap())?,
            curve: bs58_to_pubkey(key.next().unwrap())?,
            pool_fee: bs58_to_pubkey(key.next().unwrap())?,
        
        });
    }
}

/* (?)
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

impl Clob<Serum> for Serum{
    
    fn load_from_arr(keys: &[&str]) -> Result<Self>{
        keys_len_valid(keys, 17)?;
        let mut key = keys.iter();
        return Ok(Self{
            parent_program: bs58_to_pubkey(key.next().unwrap())?,
            market: bs58_to_pubkey(key.next().unwrap())?,
            market_open_orders: bs58_to_pubkey(key.next().unwrap())?,
            market_request_queue: bs58_to_pubkey(key.next().unwrap())?,
            market_event_queue: bs58_to_pubkey(key.next().unwrap())?,
            market_bids: bs58_to_pubkey(key.next().unwrap())?,
            market_asks: bs58_to_pubkey(key.next().unwrap())?,
            market_order_payer_token_account: bs58_to_pubkey(key.next().unwrap())?,
            market_coin_vault: bs58_to_pubkey(key.next().unwrap())?,
            market_pc_vault: bs58_to_pubkey(key.next().unwrap())?,
            market_vault_signer: bs58_to_pubkey(key.next().unwrap())?,
            market_coin_wallet: bs58_to_pubkey(key.next().unwrap())?,
            user_authority: bs58_to_pubkey(key.next().unwrap())?,
            user_pc_wallet: bs58_to_pubkey(key.next().unwrap())?,
            dex_program: bs58_to_pubkey(key.next().unwrap())?,
            token_program: bs58_to_pubkey(key.next().unwrap())?,
            rent: bs58_to_pubkey(key.next().unwrap())?,
        });
    }
}
