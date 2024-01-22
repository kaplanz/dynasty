#![warn(clippy::pedantic)]
#![allow(clippy::ignored_unit_patterns)]

use std::net::IpAddr;
use std::process::Command;
use std::thread::sleep;
use std::{fs, io};

use anstream::eprintln;
use anyhow::{anyhow, bail, Context, Result};
use async_compat::Compat;
use clap::Parser;
use futures::future;
use log::{debug, info, trace, warn};
use reqwest::Client;

use self::cfg::Config;
use crate::cli::Args;
use crate::sv::Provider;

mod cfg;
mod cli;
mod err;
mod sv;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{err}", err = err::fmt::plain(&err));
    }
}

fn try_main() -> Result<()> {
    // Parse args
    let args = Args::parse();
    trace!("{args:?}");
    // Initialize logging
    env_logger::builder()
        .filter_level(args.verbose.log_level_filter())
        .init();
    // Parse conf
    let conf: Config = {
        // Read file
        let path = args.conf;
        match fs::read_to_string(&path) {
            Ok(read) => Ok(read),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(String::default()),
                _ => Err(err).with_context(|| format!("could not read: `{}`", path.display())),
            },
        }
        // Parse conf
        .and_then(|read| toml::from_str(&read).context("could not parse file"))
    }
    .context("unable to load config")?;
    trace!("{conf:?}");

    // Determine iteration length
    let iter: Box<dyn Iterator<Item = ()>> = if args.mode.daemon {
        // Extract daemon duration
        let dur = conf
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
        let addr = resolve(&conf.resolver).context("failed to run resolver")?;
        // Check if public address updated
        if prev.is_some() && prev != Some(addr) {
            info!("public address changed: {addr}");
        }
        prev = Some(addr);
        // Create HTTP requests
        let mut requests: Vec<_> = conf
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
        if args.mode.dry_run {
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
            .zip(&conf.services)
            // Report this result, forwarding errors
            .map(|(res, sv)| sv.report(res?).context("failed to generate report"))
            // Report errors
            .filter_map(Result::err)
            .for_each(|err| {
                eprintln!("{err}", err = err::fmt::plain(&err));
            });
    }

    Ok(())
}

/// Resolve current server public address.
fn resolve(cmd: &String) -> Result<IpAddr> {
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
