mod db;
mod server;
mod smtp;

pub use db::DbConfig;
pub use server::ServerConfig;
pub use smtp::SmtpConfig;

pub type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug)]
pub enum ConfigError {
    NotFound(String),
    Db(String),
    Server(String),
    Smtp(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(err) => write!(f, "Missing Environment Variable: {err}"),
            Self::Db(err) => write!(f, "Database Configuration Error: {err}"),
            Self::Smtp(err) => write!(f, "SMTP Configuration Error: {err}"),
            Self::Server(err) => write!(f, "Server Configuration Error: {err}"),
        }
    }
}

impl std::error::Error for ConfigError {}
