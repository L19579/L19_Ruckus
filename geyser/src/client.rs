use {
    redis,
    anyhow::{
        anyhow, 
        Result,
    },
    std::{
        fs,
        io::prelude::*,
    },
    crate::config::RedisConfig,
};

#[derive(Clone, Debug)]
pub struct RedisClient{
    pub redis: redis::Client,
}

impl RedisClient{
    pub fn new(redis_config: RedisConfig) -> Result<Self>{

        let redis = redis::Client::
            open(redis_config.uri().as_str())
            .expect("Redis DB connection failed.");
        return Ok(Self {
            redis, 
        });
    }
}
