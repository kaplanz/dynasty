//! Command-line interface.

use std::path::PathBuf;

use clap::{Args, Parser, ValueHint};
use clap_verbosity_flag::Verbosity;

use crate::cfg::{self, Config};

/// Dynamic DNS client.
///
/// Dynasty is a dynamic DNS client written in Rust and designed to be easily
/// extensible to support any DNS provider service.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about)]
pub struct Cli {
    /// Configuration options.
    #[clap(flatten)]
    pub cfg: Settings,

    // Execution mode.
    //
    // Determine how the application should be run.
    #[clap(flatten)]
    #[clap(next_help_heading = None)]
    pub run: Runtime,

    /// Logging verbosity.
    #[clap(flatten)]
    #[clap(next_help_heading = None)]
    pub verbose: Verbosity,
}

/// Configuration options.
#[derive(Args, Debug)]
pub struct Settings {
    /// Configuration file.
    ///
    /// When options are specified in multiple locations, they will be applied
    /// with the following precedence: cli > env > file.
    #[clap(long = "conf")]
    #[clap(value_name = "PATH")]
    #[clap(default_value_os_t = cfg::path())]
    #[clap(help_heading = None)]
    #[clap(value_hint = ValueHint::FilePath)]
    pub path: PathBuf,

    /// Configuration data.
    #[clap(flatten)]
    #[clap(next_help_heading = "Settings")]
    pub data: Config,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct Runtime {
    /// Run as a daemon.
    ///
    /// Enables daemon mode, causing the program to block, periodically
    /// refreshing DNS entries as configured.
    #[clap(short, long)]
    pub daemon: bool,

    /// Perform a dry run.
    ///
    /// Run without modifying any DNS records. This is useful for testing
    /// configuration.
    #[clap(short = 'n', long)]
    #[clap(conflicts_with = "daemon")]
    pub dry_run: bool,
}
