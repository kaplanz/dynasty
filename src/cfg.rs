use std::path::PathBuf;
use std::time::Duration;

use serde::Deserialize;

use crate::sv::Service;

/// Configuration directory path.
pub fn dir() -> PathBuf {
    dirs::config_dir().unwrap().join("dynasty")
}

/// App configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Daemon mode.
    pub daemon: Option<Daemon>,
    /// Public IP address resolver command.
    #[serde(default = "Config::resolver")]
    pub resolver: String,
    /// DNS provider services.
    #[serde(default)]
    pub services: Vec<Service>,
}

impl Config {
    /// Default resolver using `opendns.com`.
    fn resolver() -> String {
        "dig @resolver4.opendns.com myip.opendns.com +short".to_string()
    }
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Daemon {
    /// Timeout after which DNS must be re-synced.
    #[serde(deserialize_with = "duration_str::deserialize_duration")]
    pub timeout: Duration,
}
