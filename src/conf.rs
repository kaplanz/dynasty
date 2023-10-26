use serde::Deserialize;

use crate::sv::Service;

/// App configuration.
#[derive(Debug, Deserialize)]
pub struct Conf {
    /// Daemon mode.
    pub daemon: Option<Daemon>,
    /// Public IP address resolver command.
    pub resolver: String,
    /// DNS provider services.
    pub services: Vec<Service>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Daemon {
    /// Timeout (in seconds) after which DNS must be re-synced.
    pub timeout: u32,
}
