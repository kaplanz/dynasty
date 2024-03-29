use std::net::IpAddr;

use log::trace;
use reqwest::{Client, Request, Response};
use serde::Deserialize;
use strum::Display;
use thiserror::Error;

use self::cloudflare::Cloudflare;

mod cloudflare;

/// Service trait.
pub trait Provider {
    type Error;

    /// Prepares a request using the provider's API.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request could not be prepered.
    fn request(&self, addr: IpAddr) -> Result<Request, Self::Error>;

    /// Reports on a response from the provider.
    ///
    /// # Errors
    ///
    /// This function will return an error if the response could not be
    /// reported.
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
    ///
    /// # Errors
    ///
    /// This function will return an error if the update failed to execute.
    #[allow(unused)]
    pub async fn update(&self, addr: IpAddr) -> Result<Response, Error> {
        // Prepare an API request to the service
        let req = self.request(addr)?;
        trace!("{req:?}");
        // Create a client
        let client = Client::new();
        // Execute the request
        client.execute(req).await.map_err(Error::Reqwest)
    }
}

impl Provider for Service {
    type Error = Error;

    fn request(&self, addr: IpAddr) -> Result<Request, Self::Error> {
        match self {
            Service::Cloudflare(sv) => sv.request(addr).map_err(Error::Cloudflare),
        }
    }

    fn report(&self, res: Response) -> Result<(), Self::Error> {
        match self {
            Service::Cloudflare(sv) => sv.report(res).map_err(Error::Cloudflare),
        }
    }
}

/// An error caused by a service.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request error.
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// Cloudflare service error.
    #[error(transparent)]
    Cloudflare(#[from] cloudflare::Error),
}
