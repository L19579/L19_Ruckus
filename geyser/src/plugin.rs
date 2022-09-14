// unwraps ok.
use {
    crate::{
        config::Config,
        client::RedisClient,
    },
    solana_geyser_plugin_interface::
    geyser_plugin_interface::{
        GeyserPlugin,
        SlotStatus,
        Result as GeyserResult,
        ReplicaAccountInfoVersions,
        ReplicaTransactionInfoVersions,
        ReplicaBlockInfoVersions,
    },
};

#[derive(Clone, Debug)]
pub struct GeyserRedisPlugin{
    pub redis_client: Option<RedisClient>,
}

impl GeyserPlugin for GeyserRedisPlugin{
    fn name(&self) -> &'static str{
        return "GeyserRedisPlugin";
    }

    fn on_load(&mut self, _config_file: &str) -> GeyserResult<()>{
        let redis_config = Config::load().unwrap().redis.unwrap();
        self.redis_client = Some(RedisClient::new(&redis_config).unwrap());
        return Ok(());
    }

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        _is_startup: bool,
    ) -> GeyserResult<()>{
        let account = match account{
            ReplicaAccountInfoVersions::V0_0_1(a) => a,
        };

        self.redis_client
            .as_mut()
            .unwrap()
            .account_event(slot, &account)
            .unwrap();

        return Ok(());
    }
    
    fn update_slot_status(
        &mut self,
        slot: u64,
        parent: Option<u64>,
        status: SlotStatus,
    ) -> GeyserResult<()>{
        self.redis_client
            .as_mut()
            .unwrap()
            .slot_update_event(slot, parent, status)
            .unwrap();

        return Ok(());
    }
    
    // Rest required for geyser interface but not used in this program.
    fn on_unload(&mut self){}
    fn notify_end_of_startup(&mut self) -> GeyserResult<()>{
        return Ok(());
    }
    fn notify_transaction(
        &mut self,
        _transaction: ReplicaTransactionInfoVersions,
        _slot: u64,
    ) -> GeyserResult<()>{
        return Ok(());
    }
    fn notify_block_metadata(
        &mut self,
        _blockinfo: ReplicaBlockInfoVersions,
    ) -> GeyserResult<()>{
        return Ok(());
    }
}
