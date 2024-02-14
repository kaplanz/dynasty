#![warn(clippy::pedantic)]

use std::net::IpAddr;
use std::process::Command;
use std::thread::sleep;

use anyhow::{anyhow, bail, Context};
use async_compat::Compat;
use clap::{crate_name as name, Parser};
use futures::future;
use log::{debug, info, trace, warn};
use reqwest::Client;

use crate::cfg::Config;
use crate::cli::Cli;
use crate::err::{Exit, Result};
use crate::sv::Provider;

pub mod cfg;
pub mod cli;
pub mod err;
pub mod sv;

/// Name of this crate.
///
/// This may be used for base subdirectories.
pub const NAME: &str = name!();

fn main() -> Exit {
    // Parse args
    let mut args = Cli::parse();
    // Load config
    args.cfg.merge({
        match Config::load(&args.conf) {
            Ok(conf) => conf,
            Err(err) => return err.into(),
        }
    });
    // Initialize logger
    env_logger::builder()
        .filter_level(args.verbose.log_level_filter())
        .init();
    // Log previous steps
    trace!("{args:?}");

    // Run application
    match run(args) {
        Ok(()) => (),
        Err(err) => return err.into(),
    }

    // Terminate normally
    Exit::Success
}

fn run(args: Cli) -> anyhow::Result<()> {
    // Determine iteration length
    let iter: Box<dyn Iterator<Item = ()>> = if args.run.daemon {
        // Extract daemon duration
        let dur = args
            .cfg
            .daemon
            .ok_or(anyhow!("daemon is not configured"))?
            .timeout;
        // Iterate with a delay
        let delay = std::iter::from_fn(move || {
            sleep(dur);
            Some(())
        });
        // First iteration should occur immediately
        let first = std::iter::once(());
        // Chain iterators together
        Box::new(first.chain(delay))
    } else {
        // Iterate exactly once
        Box::new(std::iter::once(()))
    };

    // Perform DNS updates
    let mut prev = None; // keep last public address
    for () in iter {
        // Resolve public IP address
        let addr = resolve(&args.cfg.resolver).context("failed to run resolver")?;
        // Check if public address updated
        if prev.is_some() && prev != Some(addr) {
            info!("public address changed: {addr}");
        }
        prev = Some(addr);
        // Create HTTP requests
        let mut requests: Vec<_> = args
            .cfg
            .services
            .iter()
            .map(|service| service.request(addr))
            .inspect(|req| match req {
                Ok(req) => trace!("{req:?}"),
                Err(err) => warn!("{err}"),
            })
            .filter_map(Result::ok)
            .collect();
        // Don't send any requests on a dry run
        if args.run.dry_run {
            requests.clear();
        }
        // Asynchronously update services
        let client = Client::new();
        let responses = smol::block_on(Compat::new(async {
            future::join_all(requests.into_iter().map(|req| {
                let client = &client;
                async move {
                    client
                        // Execute the request
                        .execute(req)
                        .await
                        // Report execution errors
                        .map_err(sv::Error::Reqwest)?
                        // Report API errors
                        .error_for_status()
                        .context("failed with status")
                }
            }))
            .await
        }));
        // Report service responses
        responses
            .into_iter()
            .zip(&args.cfg.services)
            // Report this result, forwarding errors
            .map(|(res, sv)| sv.report(res?).context("failed to generate report"))
            // Report errors
            .filter_map(Result::err)
            .for_each(|err| {
                advise::error!("{err:#}");
            });
    }

    Ok(())
}

/// Resolve current server public address.
fn resolve(cmd: &String) -> anyhow::Result<IpAddr> {
    // Lexically tokenize resolver command
    let mut tokens = shlex::split(cmd).ok_or(anyhow!("failed to lex resolver: `{cmd}`"))?;
    if tokens.is_empty() {
        bail!("resolver command is empty");
    }
    let (prog, args) = (tokens.remove(0), tokens);
    trace!("program: `{prog}`, args: {args:?}");
    // Query IP using external resolver
    debug!("querying public address: `{cmd}`");
    let out = Command::new(prog)
        .args(args)
        .output()
        .context("failed to execute resolver")?;
    // Parse command output into IP
    let addr = std::str::from_utf8(&out.stdout)?.trim().parse()?;
    debug!("resolved public address: {addr}");

    Ok(addr)
}
