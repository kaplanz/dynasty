use std::net::IpAddr;

use log::trace;
use reqwest::blocking::{Client, Request, Response};
use serde::Deserialize;
use strum::Display;
use thiserror::Error;

use self::cloudflare::Cloudflare;

mod cloudflare;

/// Service trait.
pub trait Provider {
    type Error;

    /// Prepares a request using the provider's API.
    fn request(&self, dns: IpAddr) -> Result<Request, Self::Error>;

    /// Reports on a response from the provider.
    fn report(&self, res: Response) -> Result<(), Self::Error>;
}

/// Service kind.
#[derive(Debug, Deserialize, Display)]
#[serde(tag = "provider")]
#[non_exhaustive]
pub enum Service {
    Cloudflare(Cloudflare),
}

impl Service {
    /// Update DNS records for this service.
    pub fn update(&self, dns: IpAddr) -> Result<Response, Error> {
        // Prepare an API request to the service
        let req = self.request(dns)?;
        trace!("{req:?}");
        // Create a client
        let client = Client::new();
        // Execute the request
        client.execute(req).map_err(Error::Reqwest)
    }
}

impl Provider for Service {
    type Error = Error;

    fn request(&self, dns: IpAddr) -> Result<Request, Self::Error> {
        match self {
            Service::Cloudflare(sv) => sv.request(dns).map_err(Error::Cloudflare),
        }
    }

    fn report(&self, res: Response) -> Result<(), Self::Error> {
        match self {
            Service::Cloudflare(sv) => sv.report(res).map_err(Error::Cloudflare),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Cloudflare(#[from] cloudflare::Error),
}
