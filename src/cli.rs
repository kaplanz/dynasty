use std::path::PathBuf;

use clap::{Parser, ValueHint};
use clap_verbosity_flag::Verbosity;

use crate::cfg;

/// Dynamic DNS Client.
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Configuration file.
    ///
    /// Path to the configuration file. Used to control services for DNS
    /// providers, define daemon parameters, and other options.
    #[arg(short, long)]
    #[arg(default_value = cfg::dir().join("config.toml").into_os_string())]
    #[arg(value_hint = ValueHint::FilePath)]
    pub conf: PathBuf,

    // Execution mode.
    //
    // Determine how the application should be run.
    #[command(flatten)]
    pub mode: Mode,

    /// Logging verbosity.
    #[command(flatten)]
    pub verbose: Verbosity,
}

#[derive(clap::Args, Debug)]
#[group(multiple = false)]
pub struct Mode {
    /// Run as a daemon.
    ///
    /// Enables daemon mode, causing the program to block, periodically
    /// refreshing DNS entries as configured.
    #[arg(short, long)]
    pub daemon: bool,

    /// Perform a dry run.
    ///
    /// Run without modifying any DNS records. This is useful for testing
    /// configuration.
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}
