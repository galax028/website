use anyhow::{Context, Result};
use dotenvy::dotenv;
use http::HeaderValue;
use std::{env::var, fs, net::IpAddr, path::PathBuf};

/// Global configuration options for the application.
#[derive(Clone)]
pub struct Config {
    pub cors_origins: Vec<HeaderValue>,
    pub database_url: String,
    pub host: IpAddr,
    pub port: u16,
    pub static_root: PathBuf,
}

impl Config {
    #[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
    /// Creates a new `Config` by loading environment variables.
    pub fn new() -> Result<Config> {
        dotenv().ok();

        let cors_origins = var("CORS_ORIGINS")
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let database_url = var("DATABASE_URL").context("DATABASE_URL must be set")?;
        let host = var("HOST").context("HOST must be set")?.parse::<IpAddr>()?;
        let port = var("PORT").context("PORT must be set")?.parse::<u16>()?;
        let static_root = var("STATIC_ROOT")
            .context("STATIC_ROOT must be set")?
            .parse::<PathBuf>()
            .unwrap(); // Unwrap-safe: Error type from result is `Infallible`.

        if !fs::metadata(&static_root)
            .context("STATIC_ROOT does not exist or cannot be accessed")?
            .is_dir()
        {
            anyhow::bail!("STATIC_ROOT must be a directory");
        }

        Ok(Config {
            cors_origins,
            database_url,
            host,
            port,
            static_root,
        })
    }
}
