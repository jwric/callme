use std::{str::FromStr, sync::Arc, time::Duration};

use anyhow::{bail, Context, Result};
use iroh::{Endpoint, EndpointAddr, SecretKey};
pub use iroh_roq::ALPN;

use crate::rtc::RtcConnection;

pub async fn bind_endpoint() -> Result<Endpoint> {
    let secret_key = match std::env::var("IROH_SECRET") {
        Ok(secret) => {
            SecretKey::from_str(&secret).expect("failed to parse secret key from IROH_SECRET")
        }
        Err(_) => SecretKey::generate(&mut rand::rng()),
    };
    Endpoint::builder()
        .secret_key(secret_key)
        .discovery(iroh::discovery::dns::DnsDiscovery::n0_dns())
        .alpns(vec![ALPN.to_vec()])
        .relay_mode(iroh::RelayMode::Disabled)
        .bind()
        .await
        .map_err(Into::into)
}
