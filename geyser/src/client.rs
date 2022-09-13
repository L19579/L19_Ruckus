use {
    redis,
    sqlx::{
        postgres,
        Executor,
    },
    anyhow::{
        anyhow, Result,
    },
    std::{
        io::prelude::*,
        fs,
    },
};

//likely to drop
const POSTGRES_LAUNCH_SCRIPT: &str = 
    "~/partition_3/programming_2/FULL/general_arb/L19_Ruckus/postgres_launch.sql";

#[derive(Clone, Debug)]
pub struct RedisClient{
    pub redis: redis::Client,
}

impl RedisClient{
    pub fn new() -> Result<Self>{
        // ----- assign uri in config file
        let redis = redis::Client::
            open("redis://jojo:password@127.0.0.1:6379/ruckusdb")
            .expect("Redis DB connection failed.");
        return Ok(Self {
            redis, 
        });
    }
}

#[derive(Clone, Debug)]
pub struct PostgresClient{
    pub postgres: postgres::PgPool,
}

impl PostgresClient{
    pub async fn new() -> Result<Self>{
        // ----- check that server is online and db/tables configured w/ sql script pre launch
        let postgres_script: String = fs::read_to_string(POSTGRES_LAUNCH_SCRIPT)?;
        // ----- assign uri in config file
        let postgres = postgres::PgPool::
            connect("postgresql://jojo:password@127.0.0.1:5432/ruckusdb")
            .await
            .expect("Postgres DB connection failed.");
        postgres.execute(postgres_script.as_str())
            .await
            .expect("Failed to execute Postgres startup script.");

        return Ok(Self{
            postgres
        }); 
    }
    
    pub fn load_target_accounts() -> Result<()>{
        return Ok(());
    }
}
