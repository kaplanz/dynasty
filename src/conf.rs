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
pub struct Conf {
    /// Daemon mode.
    pub daemon: Option<Daemon>,
    #[serde(default = "Conf::resolver")]
    /// Public IP address resolver command.
    pub resolver: String,
    /// DNS provider services.
    #[serde(default)]
    pub services: Vec<Service>,
}

impl Conf {
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
