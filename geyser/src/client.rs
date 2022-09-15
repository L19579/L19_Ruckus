use {
    redis,
    anyhow::Result,
    crate::config::RedisConfig, 
    solana_sdk::{
        pubkey::Pubkey,
        message::SanitizedMessage,
    },
    solana_geyser_plugin_interface::
    geyser_plugin_interface::{
        SlotStatus,
        ReplicaAccountInfo,
        ReplicaTransactionInfo,
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

    pub fn account_event(
        &mut self, 
        slot: u64, 
        account: &ReplicaAccountInfo
    ) -> Result<()>{
        let key = format!("account.{}", Pubkey::new(account.pubkey));
        redis::cmd("HSET").arg(&key)
            .arg("slot").arg(slot)
            .arg("lamports").arg(account.lamports)
            .arg("owner").arg(Pubkey::new(account.owner).to_string())
            .arg("executable").arg(account.executable)
            .arg("rent_epoch").arg(account.rent_epoch)
            .arg("data").arg(account.data)
            .arg("write_version").arg(account.write_version)
            .execute(&mut self.redis.get_connection()?);

        return Ok(());
    }
    
    pub fn transaction_event(
        &mut self, 
        slot: u64, 
        transaction: &ReplicaTransactionInfo
    ) -> Result<()>{
        let (m_account_keys, m_instructions) = 
            match transaction.transaction.message(){
            SanitizedMessage::Legacy(m) => {
                (&m.account_keys, &m.instructions) 
            },
            SanitizedMessage::V0(m) => { // like inactive on 1.10.35
                (&m.message.account_keys, &m.message.instructions) 
            }, 
        };
        
        let key = format!("transaction.{}", transaction.signature);
        let mut db_cmd = redis::Cmd::new();
        db_cmd.arg("HSET").arg(&key)
            .arg("slot").arg(slot)
            .arg("n_account_keys")
            .arg(m_account_keys.len())
            .arg("n_instructions")
            .arg(m_instructions.len());
       
        m_account_keys.iter().enumerate().for_each(|(i, pubkey)|{
            db_cmd.arg(&format!("pubkey_{}", i))
                .arg(pubkey.to_string());
        });
        
        m_instructions.iter().enumerate().for_each(|(i, instruction)|{
            let prefix = format!("instruction_{}", i);
            db_cmd.arg(&format!("{}.program_id_index", prefix))
                .arg(instruction.program_id_index)
                .arg(&format!("{}.accounts.", prefix))
                .arg(&instruction.accounts)
                .arg(&format!("{}.data", prefix)) // not req atm but cheap
                .arg(&instruction.data);
        }); 
        db_cmd.execute(&mut self.redis.get_connection()?);

        return Ok(());
    }
    
    pub fn slot_update_event(
        &mut self, 
        slot: u64, parent: Option::<u64>, 
        status: SlotStatus
    ) -> Result<()>{
        let key = format!("slot.{}", slot);
        redis::cmd("HSET").arg(&key)
            .arg("parent").arg(parent.unwrap_or(0)) // zero = none
            .arg("status").arg(status.as_str())
            .execute(&mut self.redis.get_connection()?);

        return Ok(());
    }
}
