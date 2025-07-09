use std::env;

use super::{ConfigError, ConfigResult};

#[derive(Debug)]
pub struct SmtpConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    from_email: String,
    from_name: String,
}

impl SmtpConfig {
    pub fn with_prefix(prefix: &str) -> ConfigResult<Self> {
        let host = env::var(format!("{prefix}HOST"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}HOST' not found")))?;
        let port = env::var(format!("{prefix}PORT"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}PORT' not found")))?
            .parse::<u16>()
            .map_err(|_| ConfigError::Smtp(format!("Invalid Port: {prefix}PORT")))?;
        let username = env::var(format!("{prefix}USERNAME"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}USERNAME' not found")))?;
        let password = env::var(format!("{prefix}PASSWORD"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}PASSWORD' not found")))?;
        let from_email = env::var(format!("{prefix}FROM_EMAIL"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}FROM_EMAIL' not found")))?;
        let from_name = env::var(format!("{prefix}FROM_NAME"))
            .map_err(|_| ConfigError::NotFound(format!("'{prefix}FROM_NAME' not found")))?;
        Ok(Self {
            host,
            port,
            username,
            password,
            from_email,
            from_name,
        })
    }

    pub fn get_host(&self) -> &str {
        &self.host
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

    pub fn get_from_email(&self) -> &str {
        &self.from_email
    }

    pub fn get_from_name(&self) -> &str {
        &self.from_name
    }
}
