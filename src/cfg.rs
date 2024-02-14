//! Application configuration.

use std::io::ErrorKind::NotFound;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, io};

use clap::Args;
use serde::Deserialize;
use thiserror::Error;
use toml::from_str as parse;

use crate::sv::Service;
use crate::NAME;

/// Configuration data.
#[derive(Args, Debug, Deserialize)]
pub struct Config {
    /// Daemon mode.
    #[clap(skip)]
    pub daemon: Option<Daemon>,

    /// Public IP address resolver command.
    #[clap(skip)]
    pub resolver: Resolver,

    /// DNS provider services.
    #[clap(skip)]
    #[serde(default)]
    pub services: Vec<Service>,
}

/// Resolver command.
#[derive(Debug, Deserialize)]
pub struct Resolver(String);

impl Deref for Resolver {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self("dig @resolver4.opendns.com myip.opendns.com +short".into())
    }
}

/// Daemon configuration.
#[derive(Debug, Deserialize)]
pub struct Daemon {
    /// Timeout after which DNS must be re-synced.
    #[serde(deserialize_with = "duration_str::deserialize_duration")]
    pub timeout: Duration,
}

impl Config {
    /// Returns the path to the application's configuration file.
    #[must_use]
    pub fn path() -> PathBuf {
        xdir::config()
            .map(|path| path.join(NAME))
            .unwrap_or_default()
            .join("config.toml")
    }

    /// Loads configuration data from a file.
    ///
    /// # Errors
    ///
    /// This function will return an error if the configuration could not be
    /// loaded.
    pub fn load(path: &Path) -> Result<Self, Error> {
        match fs::read_to_string(path) {
            // If the configuration file does not exist, return an empty string,
            // resulting in all fields being populated with defaults.
            Err(err) if err.kind() == NotFound => Ok(String::default()),
            // For other errors, return them directly.
            Err(err) => Err(err.into()),
            // On success, return the body of the file can be parsed.
            Ok(body) => Ok(body),
        }
        .and_then(|body| {
            // If a configuration file was read, parse it.
            parse(&body)
                // Parsing errors should be mapped into a separate variant.
                .map_err(Into::into)
        })
    }

    /// Combines two configuration instances.
    ///
    /// This is useful when some configurations may also be supplied on the
    /// command-line. When merging, it is best practice to prioritize options
    /// from the cli to those saved on-disk. To do so, prefer keeping data
    /// fields from `self` when conflicting with `other`.
    pub fn merge(&mut self, other: Self) {
        drop(other);
    }
}

/// An error caused by [loading][`Config::load`] configuration.
#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read config")]
    Read(#[from] io::Error),
    #[error("failed to parse config")]
    Parse(#[from] toml::de::Error),
}
