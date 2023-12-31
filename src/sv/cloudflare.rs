use std::net::IpAddr;

use log::debug;
use reqwest::{Body, Method, Request, Response};
use serde::Deserialize;
use serde_json::{json, Value};
use thiserror::Error;

use super::Provider;

#[derive(Debug, Deserialize)]
pub struct Cloudflare {
    /// API token.
    token: String,
    /// Zone identifier.
    zone: String,
    /// Managed domain identifiers.
    record: String,
}

impl Provider for Cloudflare {
    type Error = Error;

    fn request(&self, addr: IpAddr) -> Result<Request, Self::Error> {
        // Extract parts
        let Self {
            token,
            zone,
            record,
        } = self;
        // Declare request
        let method = Method::PATCH;
        let url = format!("https://api.cloudflare.com/client/v4/zones/{zone}/dns_records/{record}")
            .parse()?;
        let mut req = Request::new(method, url);
        // Add headers
        let headers = req.headers_mut();
        headers.insert("Authorization", format!("Bearer {token}").parse()?);
        headers.insert("Content-Type", "application/json".parse().unwrap());
        // Attach data
        let data = json!({
            "content": addr,
        });
        *req.body_mut() = Some(Body::from(data.to_string()));

        Ok(req)
    }

    fn report(&self, res: Response) -> Result<(), Self::Error> {
        // Extract results from response
        let res: serde_json::Value = smol::block_on(async { res.json().await })?;
        let res = res
            .get("result")
            .ok_or_else(|| Error::Report(res.clone()))?;
        // Report attributes
        let name = res
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::Report(res.clone()))?;
        let kind = res
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::Report(res.clone()))?;
        let addr = res
            .get("content")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::Report(res.clone()))?;
        debug!("updated {name}: {kind} record: {addr}");

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Header(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("cannot parse response: {0:?}")]
    Report(serde_json::Value),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
}
