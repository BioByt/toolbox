use std::{env, net::IpAddr, time::Duration};

#[derive(Debug)]
pub struct DbConfig {
    host: IpAddr,
    port: u16,
    min_connections: u32,
    max_connections: u32,
    idle_timeout: Duration,
}

impl DbConfig {
    pub fn from_env(prefix: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let host = env::var(format!("{prefix}HOST"))?.parse::<IpAddr>()?;
        let port = env::var(format!("{prefix}PORT"))?.parse::<u16>()?;
        let min_connections = env::var(format!("{prefix}MIN_CONNECTIONS"))?.parse::<u32>()?;
        let max_connections = env::var(format!("{prefix}MAX_CONNECTIONS"))?.parse::<u32>()?;
        let idle_timeout =
            Duration::from_secs(env::var(format!("{prefix}IDLE_TIMEOUT"))?.parse::<u64>()?);
        Ok(Self {
            host,
            port,
            min_connections,
            max_connections,
            idle_timeout,
        })
    }

    pub fn get_database_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
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
}
