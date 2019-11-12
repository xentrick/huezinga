use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Deserialize)]
pub struct Nupnp {
    id: String,
    pub internalipaddress: IpAddr,
}
