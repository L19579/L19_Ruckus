use {
    redis,
    // self for ::Client
    // eliminate tokio as subdepency.
    // sqlx' PgPool should allow copy with 'std' futures
    postgres::{ 
        self, NoTls,
    },
    sqlx::postgres 
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

#[derive(Clone, Debug)]
pub struct PostgresClient{
    pub postgres: postgres::Client,
}

impl PostgresClient{
    pub fn new() -> Result<Self>{
        let mut postgres = postgres::
            Client::connect("host=localhost user=postgres", NoTls)?;            
        let postgres_script: String = fs::read_to_string(POSTGRES_LAUNCH_SCRIPT)?;

        postgres.batch_execute(&postgres_script)?;
        return Ok(Self{
            postgres
        }); 
    }
}
