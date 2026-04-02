use reqwest::Client;
use serde::de::DeserializeOwned;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    ops::RangeInclusive,
    time::Duration,
};
use thiserror::Error;

use crate::response::{Empty, InfosData, Response};

pub mod key;
pub mod response;

const DEFAULT_CLIENT_TIMEOUT_MS: u64 = 250;

const MODE_PRESS: u8 = 0;
const MODE_HOLD: u8 = 1;
const MODE_RELEASE: u8 = 2;

#[derive(Debug, Error)]
pub enum DecoderError {
    #[error("Connection failed to {0}")]
    ConnectionFailed(SocketAddrV4),

    #[error("Request timed out on {0}")]
    Timeout(SocketAddrV4),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),
}

/// Builds an HTTP client with the specified timeout.
///
/// # Arguments
///
/// * `millis` - Timeout in milliseconds.
///
/// # Panics
///
/// Panics if the underlying HTTP client fails to initialize.
fn build_client(millis: u64) -> Client {
    let duration = Duration::from_millis(millis);
    Client::builder().timeout(duration).build().unwrap()
}

pub struct Decoders;

impl Decoders {
    /// Opens a connection to a decoder at the given socket address.
    ///
    /// # Arguments
    ///
    /// * `socket` - Target socket address.
    pub fn connect(ip: impl Into<Ipv4Addr>, port: u16) -> Decoder {
        let socket = SocketAddrV4::new(ip.into(), port);
        Decoder::new(socket)
    }

    /// Prepares a network scan over the given IP and port ranges.
    ///
    /// Call [`DecoderSearch::find`] to execute the scan and retrieve responding decoders.
    ///
    /// # Arguments
    ///
    /// * `ips` - Range of IPv4 addresses to scan.
    /// * `ports` - Range of ports to probe on each address.
    pub fn search(ips: RangeInclusive<Ipv4Addr>, ports: RangeInclusive<u8>) -> DecoderSearch {
        DecoderSearch::new(ips, ports)
    }
}

pub struct DecoderSearch {
    ips: RangeInclusive<Ipv4Addr>,
    ports: RangeInclusive<u8>,
    client: Client,
}

impl DecoderSearch {
    /// Creates a new scan over the given IP and port ranges.
    ///
    /// # Arguments
    ///
    /// * `ips` - Range of IPv4 addresses to scan.
    /// * `ports` - Range of ports to probe on each address.
    fn new(ips: RangeInclusive<Ipv4Addr>, ports: RangeInclusive<u8>) -> Self {
        let client = build_client(DEFAULT_CLIENT_TIMEOUT_MS);
        Self { ips, ports, client }
    }

    /// Sets the timeout used when probing each address.
    ///
    /// # Arguments
    ///
    /// * `millis` - Timeout in milliseconds.
    pub fn with_timeout(mut self, millis: u64) -> Self {
        self.client = build_client(millis);
        self
    }

    /// Executes the scan and returns all reachable decoders found in the configured range.
    pub fn find(&self) -> Vec<Decoder> {
        todo!();
    }
}

pub struct Decoder {
    socket: SocketAddrV4,
    client: Client,
}

impl Decoder {
    /// Creates a decoder targeting the given socket address.
    ///
    /// # Arguments
    ///
    /// * `socket` - Target socket address.
    fn new(socket: impl Into<SocketAddrV4>) -> Self {
        let socket = socket.into();
        let client = build_client(DEFAULT_CLIENT_TIMEOUT_MS);
        Self { socket, client }
    }

    /// Sets the timeout used for requests to this decoder.
    ///
    /// # Arguments
    ///
    /// * `millis` - Timeout in milliseconds.
    pub fn with_timeout(mut self, millis: u64) -> Self {
        self.client = build_client(millis);
        self
    }

    pub async fn infos(&self) -> Result<InfosData, DecoderError> {
        self.request::<InfosData>(10, 0, 0).await
    }

    pub async fn press(&self, key: impl Into<u16>) -> Result<Empty, DecoderError> {
        self.request(1, key.into(), MODE_PRESS).await
    }

    pub async fn hold(&self, key: impl Into<u16>) -> Result<Empty, DecoderError> {
        self.request(1, key.into(), MODE_HOLD).await
    }

    pub async fn release(&self, key: impl Into<u16>) -> Result<Empty, DecoderError> {
        self.request(1, key.into(), MODE_RELEASE).await
    }

    async fn request<D: DeserializeOwned>(
        &self,
        operation: u8,
        key: u16,
        mode: u8,
    ) -> Result<D, DecoderError> {
        let url = format!(
            "http://{}/remoteControl/cmd?operation={}&key={}&mode={}",
            self.socket, operation, key, mode
        );
        let request = self.client.get(url).send();
        let result = request.await?.json::<Response<D>>().await?.result;
        if result.response_code != "0" {
            return Err(DecoderError::InvalidResponse(result.message));
        }
        Ok(result.data)
    }
}
