use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use super::{ConfigError, ConfigResult};

#[derive(Debug)]
pub struct ServerConfig {
    host: IpAddr,
    port: u16,
}

impl ServerConfig {
    pub fn with_prefix(prefix: &str) -> ConfigResult<Self> {
        let host = env::var(format!("{prefix}HOST"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}HOST' not found")))?
            .parse::<IpAddr>()
            .map_err(|_| ConfigError::Db(format!("Invalid IP: {prefix}HOST")))?;
        let port = env::var(format!("{prefix}PORT"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}PORT' not found")))?
            .parse::<u16>()
            .map_err(|_| ConfigError::Smtp(format!("Invalid Port: {prefix}PORT")))?;
        Ok(Self { host, port })
    }

    pub fn get_socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }

    pub fn get_host(&self) -> IpAddr {
        self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}
