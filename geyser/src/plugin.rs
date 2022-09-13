pub use {
    crate::{
        client::{
            RedisClient,
            PostgresClient,
        },
    },
    solana_geyser_plugin_interface::
        geyser_plugin_interface::{
            GeyserPlugin,
            SlotStatus,
            Result as GeyserResult,
            ReplicaAccountInfo,
            ReplicaAccountInfoVersions,
            ReplicaTransactionInfoVersions,
            ReplicaBlockInfoVersions,
        },
};

#[derive(Clone, Debug)]
pub struct GeyserRedisPlugin{
    pub redis_client: Option<RedisClient>,
    postgres_client: Option<PostgresClient>,
}

impl GeyserPlugin for GeyserRedisPlugin{
    fn name(&self) -> &'static str{
        return "GeyserRedisPlugin";
    }

    fn on_load(&mut self, _config_file: &str) -> GeyserResult<()>{
        return Ok(());
    }

    fn on_unload(&mut self){}

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> GeyserResult<()>{
        // --
        
        return Ok(());
    }

    fn notify_end_of_startup(&mut self) -> GeyserResult<()>{
        return Ok(());
    }

    fn update_slot_status(
        &mut self,
        slot: u64,
        parent: Option<u64>,
        status: SlotStatus,
    ) -> GeyserResult<()>{
        // -- 
        return Ok(());
    }

    fn notify_transaction(
        &mut self,
        transaction: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> GeyserResult<()>{
        return Ok(());
    }

    /// ignored, non critical
    fn notify_block_metadata(
        &mut self,
        blockinfo: ReplicaBlockInfoVersions,
    ) -> GeyserResult<()>{
        return Ok(());
    }
}
