use std::{env, net::IpAddr, time::Duration};

use super::{ConfigError, ConfigResult};

#[derive(Debug)]
pub struct DbConfig {
    host: IpAddr,
    port: u16,
    min_connections: u32,
    max_connections: u32,
    idle_timeout: Duration,
    username: String,
    password: String,
    database: String,
}

impl DbConfig {
    pub fn with_prefix(prefix: &str) -> ConfigResult<Self> {
        let host = env::var(format!("{prefix}HOST"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}HOST' not found")))?
            .parse::<IpAddr>()
            .map_err(|_| ConfigError::Db(format!("Invalid IP: {prefix}HOST")))?;
        let port = env::var(format!("{prefix}PORT"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}PORT' not found")))?
            .parse::<u16>()
            .map_err(|_| ConfigError::Db(format!("Invalid Integer: {prefix}PORT")))?;
        let min_connections = env::var(format!("{prefix}MIN_CONNECTIONS"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}MIN_CONNECTIONS' not found")))?
            .parse::<u32>()
            .map_err(|_| ConfigError::Db(format!("Invalid Integer: {prefix}MIN_CONNECTIONS")))?;
        let max_connections = env::var(format!("{prefix}MAX_CONNECTIONS"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}MAX_CONNECTIONS' not found")))?
            .parse::<u32>()
            .map_err(|_| ConfigError::Db(format!("Invalid Integer: {prefix}MAX_CONNECTIONS")))?;
        let idle_timeout = Duration::from_secs(
            env::var(format!("{prefix}IDLE_TIMEOUT"))
                .map_err(|_| ConfigError::NotFound(format!("'{prefix}IDLE_TIMEOUT' not found")))?
                .parse::<u64>()
                .map_err(|_| ConfigError::Db(format!("Invalid Integer: {prefix}IDLE_TIMEOUT")))?,
        );
        let username = env::var(format!("{prefix}USERNAME"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}USERNAME' not found")))?;
        let password = env::var(format!("{prefix}PASSWORD"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}PASSWORD' not found")))?;
        let database = env::var(format!("{prefix}DATABASE"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}PASSWORD' not found")))?;
        Ok(Self {
            host,
            port,
            min_connections,
            max_connections,
            idle_timeout,
            username,
            password,
            database,
        })
    }

    pub fn get_database_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    pub fn get_minimum_connections(&self) -> u32 {
        self.min_connections
    }

    pub fn get_maximum_connections(&self) -> u32 {
        self.max_connections
    }

    pub fn get_idle_timeout_duration(&self) -> Duration {
        self.idle_timeout
    }

    pub fn get_host(&self) -> IpAddr {
        self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }
}
