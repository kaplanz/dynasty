use std::path::PathBuf;

use clap::{Parser, ValueHint};

/// Dynamic DNS Client.
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Configuration file.
    ///
    /// Path to the configuration file. Used to control services for DNS
    /// providers, define daemon parameters, and other options.
    #[arg(short, long)]
    #[arg(default_value = "config.toml")]
    #[arg(value_hint = ValueHint::FilePath)]
    pub conf: PathBuf,

    // Execution mode.
    //
    // Determine how the application should be run.
    #[clap(flatten)]
    pub mode: Mode,
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
