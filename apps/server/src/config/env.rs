use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_name: String,
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::dotenv()?;
        Ok(Self {
            app_name: env::var("APP_NAME")?,
            app_host: env::var("APP_HOST")?,
            app_port: env::var("APP_PORT")?.parse()?,
            database_url: env::var("DATABASE_URL")?,
        })
    }
}
