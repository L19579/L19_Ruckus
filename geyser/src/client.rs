use {
    redis,
    anyhow::Result,
    crate::config::RedisConfig, 
    solana_sdk::pubkey::Pubkey,
    solana_geyser_plugin_interface::
    geyser_plugin_interface::{
        SlotStatus,
        ReplicaAccountInfo,
    },
};

#[derive(Clone, Debug)]
pub struct RedisClient{
    pub redis: redis::Client,
}

impl RedisClient{
    pub fn new(redis_config: &RedisConfig) -> Result<Self>{
        let redis = redis::Client::
            open(redis_config.uri().as_str())
            .expect("Redis DB connection failed.");
        return Ok(Self{
            redis, 
        });
    }

    pub fn account_event(&mut self, slot: u64, account: &ReplicaAccountInfo) -> Result<()>{
        let key = format!("account.{}", Pubkey::new(account.pubkey));
        redis::cmd("HSET").arg(&key)
            .arg("lamports").arg(account.lamports)
            .arg("owner").arg(Pubkey::new(account.owner).to_string())
            .arg("executable").arg(account.executable)
            .arg("rent_epoch").arg(account.rent_epoch)
            .arg("data").arg(account.data)
            .arg("write_version").arg(account.write_version)
            .arg("slot").arg(slot)
            .execute(&mut self.redis.get_connection()?);
        return Ok(());
    }

    pub fn slot_update_event(&mut self, slot: u64, parent: Option::<u64>, status: SlotStatus) 
    -> Result<()>{
        let key = format!("slot.{}", slot);
        redis::cmd("HSET").arg(&key)
            .arg("parent").arg(parent.unwrap_or(0)) // zero == none
            .arg("status").arg(status.as_str())
            .execute(&mut self.redis.get_connection()?);
        return Ok(());
    }
}
