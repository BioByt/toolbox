use std::{
    env,
    net::{IpAddr, SocketAddr},
};

#[derive(Debug)]
pub struct ServerConfig {
    host: IpAddr,
    port: u16,
}

impl ServerConfig {
    pub fn from_env(prefix: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let host = env::var(format!("{prefix}HOST"))?.parse::<IpAddr>()?;
        let port = env::var(format!("{prefix}PORT"))?.parse::<u16>()?;
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
