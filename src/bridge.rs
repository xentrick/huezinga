use failure::Error;
use std::net::IpAddr;
use reqwest::{Client, Response};
use reqwest::StatusCode;

use crate::json::*;


#[derive(Debug)]
/// The bridge connection
pub struct Bridge {
    client: Client,
    pub ip: Option<IpAddr>,
}

impl Bridge {
    /// Attempts to discover bridges using `https://www.meethue.com/api/nupnp`
    pub fn new() -> Self {
        Bridge {
            client: Client::new(),
            ip: None,
        }
    }

    pub async fn discover(&mut self) -> Result<IpAddr, Error> {

        let res = self.client
            .get("https://www.meethue.com/api/nupnp")
            .send()
            .await?;

        let json: Vec<Nupnp> = res.json().await?;
        self.ip = Some(json[0].internalipaddress);
        Ok(self.ip.unwrap())
    }
}
