//! Application configuration.

use std::convert::Infallible;
use std::io::ErrorKind::NotFound;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use std::{fs, io};

use clap::Args;
use serde::Deserialize;
use thiserror::Error;
use toml::from_str as parse;

use crate::sv::Service;
use crate::NAME;

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
pub fn load(path: &Path) -> Result<Config> {
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

/// Configuration data.
#[derive(Args, Debug, Deserialize)]
pub struct Config {
    /// Daemon mode.
    #[clap(skip)]
    pub daemon: Option<Daemon>,

    /// Address resolution command.
    ///
    /// The command to be used in resolving the host's public IP address.
    #[clap(short, long)]
    #[clap(value_name = "CMD")]
    pub resolver: Option<Resolver>,

    /// DNS provider services.
    #[clap(skip)]
    #[serde(default)]
    pub services: Vec<Service>,
}

/// Resolver command.
#[derive(Clone, Debug, Deserialize)]
pub struct Resolver(String);

impl Deref for Resolver {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Resolver {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

impl Default for Resolver {
    fn default() -> Self {
        "dig @resolver4.opendns.com myip.opendns.com +short"
            .parse()
            .unwrap()
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
    /// Combines two configuration instances.
    ///
    /// This is useful when some configurations may also be supplied on the
    /// command-line. When merging, it is best practice to prioritize options
    /// from the cli to those saved on-disk. To do so, prefer keeping data
    /// fields from `self` when conflicting with `other`.
    pub fn merge(&mut self, other: Self) {
        self.daemon = other.daemon;
        self.resolver = self.resolver.take().or(other.resolver);
        self.services = other.services;
    }
}

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by [loading][`Config::load`] configuration.
#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read config")]
    Read(#[from] io::Error),
    #[error("failed to parse config")]
    Parse(#[from] toml::de::Error),
}
