use std::env;

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
    pub fn from_env(prefix: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let host = env::var(format!("{prefix}HOST"))?;
        let port = env::var(format!("{prefix}PORT"))?.parse::<u16>()?;
        let username = env::var(format!("{prefix}USERNAME"))?;
        let password = env::var(format!("{prefix}PASSWORD"))?;
        let from_email = env::var(format!("{prefix}FROM_EMAIL"))?;
        let from_name = env::var(format!("{prefix}FROM_NAME"))?;
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
