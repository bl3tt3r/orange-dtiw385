use reqwest::Client;
use serde::de::DeserializeOwned;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    ops::{Deref, RangeInclusive},
    sync::Arc,
    time::Duration,
};
use thiserror::Error;
use tokio::{
    sync::{Semaphore, mpsc},
    task::JoinSet,
};

use crate::response::{Empty, InfosData, Response};

pub mod key;
pub mod response;

const DEFAULT_CLIENT_TIMEOUT_MS: u64 = 250;
const DEFAULT_CONCURRENCY: usize = 50;

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
    pub fn search(
        ips: RangeInclusive<impl Into<Ipv4Addr>>,
        ports: RangeInclusive<impl Into<u16>>,
    ) -> DecoderSearch {
        let ips = {
            let (start, end) = ips.into_inner();
            start.into()..=end.into()
        };
        let ports = {
            let (start, end) = ports.into_inner();
            start.into()..=end.into()
        };
        DecoderSearch::new(ips, ports)
    }
}

pub struct DecoderSearch {
    ips: RangeInclusive<Ipv4Addr>,
    ports: RangeInclusive<u16>,
    client: Client,
    concurrency: usize,
}

impl DecoderSearch {
    /// Creates a new scan over the given IP and port ranges.
    ///
    /// # Arguments
    ///
    /// * `ips` - Range of IPv4 addresses to scan.
    /// * `ports` - Range of ports to probe on each address.
    fn new(ips: RangeInclusive<Ipv4Addr>, ports: RangeInclusive<u16>) -> Self {
        let client = build_client(DEFAULT_CLIENT_TIMEOUT_MS);
        let concurrency = DEFAULT_CONCURRENCY;
        Self {
            ips,
            ports,
            client,
            concurrency,
        }
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

    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Executes the scan and returns all reachable decoders found in the configured range.
    pub fn find(&self) -> mpsc::Receiver<Decoder> {
        let (tx, rx) = mpsc::channel(32);
        let start = u32::from(*self.ips.start());
        let end = u32::from(*self.ips.end());
        let ports = self.ports.clone();
        let client = self.client.clone();
        let sem = Arc::new(Semaphore::new(self.concurrency));

        tokio::spawn(async move {
            let mut tasks = JoinSet::new();

            for ip_u32 in start..=end {
                let ip = Ipv4Addr::from(ip_u32);
                for port in ports.clone() {
                    let tx = tx.clone();
                    let client = client.clone();
                    let sem = sem.clone();
                    tasks.spawn(async move {
                        let _ = sem.acquire_owned().await.unwrap();

                        let socket = SocketAddrV4::new(ip, port);
                        let url = format!(
                            "http://{}/remoteControl/cmd?operation=10&key=0&mode=0",
                            socket
                        );
                        if client.get(&url).send().await.is_ok() {
                            let decoder = Decoder { socket, client };
                            let _ = tx.send(decoder).await;
                        }
                        // _permit droppé ici → libère un slot pour la prochaine tâche
                    });
                }
            }

            while tasks.join_next().await.is_some() {}
        });

        rx
    }
}

pub struct Decoder {
    socket: SocketAddrV4,
    client: Client,
}

impl Deref for Decoder {
    type Target = SocketAddrV4;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
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
