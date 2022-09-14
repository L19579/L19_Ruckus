use {
    toml,
    std::fs,
    anyhow::Result,
    serde::Deserialize,
};

// -- relative link issue
const PATH_DB_CONFIG: &str = "/home/jojo/partition_3/programming_2/FULL/general_arb/L19_Ruckus/db";

#[derive(Deserialize, Clone, Debug)]
pub struct RedisConfig{
    pub host: String,
    pub port: u64,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl RedisConfig{
    pub fn uri(&self) -> String{
        return format!("redis://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database
        ); 
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct PostgresConfig{
    pub host: String,
    pub port: u64,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl PostgresConfig{
    pub fn uri(&self) -> String{
        return format!("postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database
        ); 
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config{
    pub redis: Option<RedisConfig>,
    pub postgres: Option<PostgresConfig>,
}

impl Config{
    pub fn load() -> Result<Config>{
        let mut config_content: String = fs::read_to_string(PATH_DB_CONFIG)?;
        let config: Config = toml::de::from_str(&config_content)?;
        return Ok(config);
    }
}
