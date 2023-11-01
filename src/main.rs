#![warn(clippy::pedantic)]
#![allow(clippy::ignored_unit_patterns)]

use std::process::Command;
use std::{fs, io};

use anstream::eprintln;
use anyhow::{bail, Context, Result};
use clap::Parser;
use log::{debug, info, trace, warn};

use self::conf::Conf;
use crate::cli::Args;
use crate::sv::Provider;

mod cli;
mod conf;
mod err;
mod sv;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{err}", err = err::fmt::plain(&err));
    }
}

fn try_main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    // Parse args
    let args = Args::parse();
    trace!("{args:?}");
    // Parse conf
    let conf: Conf = {
        // Read file
        let path = args.conf;
        match fs::read_to_string(&path) {
            Ok(read) => Ok(read),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(String::default()),
            err => err.with_context(|| format!("could not read: `{}`", path.display())),
        }
        // Parse conf
        .and_then(|read| toml::from_str(&read).context("could not parse file"))
    }
    .context("unable to load config")?;
    trace!("{conf:?}");

    // Warn about unimplemented options
    if args.mode.dry_run {
        bail!("`--dry-run` option is unimplemented");
    }
    if args.mode.daemon {
        bail!("`--daemon` option is unimplemented");
    }

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
        .context("failed to execute resolver")?;
    let addr = std::str::from_utf8(&out.stdout)?.parse()?;
    info!("resolved public address: {addr}");
    // Update each service
    conf.services.iter().try_for_each(|service| {
        let res = service
            // Perform the update
            .update(addr)
            // Report configuration errors
            .with_context(|| format!("misconfigured service: {service}"))?
            // Report API errors
            .error_for_status()
            .with_context(|| format!("failed with status: {service}"))?;
        trace!("{res:?}");
        // Report response
        service.report(res).context("failed to generate report")?;

        Ok(())
    })
}
