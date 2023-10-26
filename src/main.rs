#![warn(clippy::pedantic)]

use std::fs;
use std::process::Command;

use clap::Parser;
use log::{debug, info, trace};
use narrate::{CliError, ErrorWrap, ExitCode, Result};

use self::conf::Conf;
use crate::cli::Args;
use crate::sv::Provider;

mod cli;
mod conf;
mod sv;

fn main() {
    if let Err(err) = app() {
        narrate::report::err_full(&err);
        std::process::exit(err.exit_code())
    }
}

fn app() -> Result<()> {
    // Initialize logging
    env_logger::init();
    // Parse args
    let args = Args::parse();
    trace!("{args:?}");
    // Parse conf
    let conf: Conf = {
        // Read file
        let path = args.conf;
        let read = fs::read_to_string(&path).wrap(CliError::ReadFile(path))?;
        // Parse conf
        toml::from_str(&read)
    }
    .wrap(CliError::Config)?;
    trace!("{conf:?}");

    // Perform an update
    update(&conf)?;

    Ok(())
}

fn update(conf: &Conf) -> Result<()> {
    // Resolve public IP address
    debug!("querying public address: `{cmd}`", cmd = conf.resolver);
    let out = Command::new("sh")
        .arg("-c")
        .arg(&conf.resolver)
        .output()
        .wrap("resolver failed")?;
    let addr = std::str::from_utf8(&out.stdout)?.parse()?;
    info!("resolved public address: {addr}");
    // Update each service
    conf.services.iter().try_for_each(|service| {
        let res = service
            // Perform the update
            .update(addr)
            // Report configuration errors
            .wrap_with(|| format!("configuration: {service}"))?
            // Report API errors
            .error_for_status()
            .wrap_with(|| format!("status error: {service}"))?;
        trace!("{res:?}");
        // Report response
        service.report(res).wrap("reporting error")?;

        Ok(())
    })
}
