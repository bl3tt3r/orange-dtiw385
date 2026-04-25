use crate::response::{ApiEmpty, ApiInfosData, ApiResponse};
use reqwest::Client;
#[cfg(feature = "serializable")]
use serde::ser::SerializeStruct;
use serde::{Serialize, de::DeserializeOwned};
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
use tracing::{debug, info, warn};

pub mod key;
pub mod response;

const DEFAULT_CLIENT_TIMEOUT_MS: u64 = 250;
const DEFAULT_CONCURRENCY: usize = 20;

const MODE_PRESS: u8 = 0;
const MODE_HOLD: u8 = 1;
const MODE_RELEASE: u8 = 2;

/// Errors that can occur when communicating with a decoder.
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

/// Builds an HTTP client with the given request timeout.
fn build_client(millis: u64) -> Client {
    Client::builder()
        .timeout(Duration::from_millis(millis))
        .build()
        .expect("Unable to initialize a new client.")
}

/// Entry point for connecting to or searching for decoders on the network.
pub struct Decoders;

impl Decoders {
    /// Opens a connection to a decoder at the given address.
    ///
    /// This does not perform any network call — use [`Decoder::infos`] or
    /// [`Decoder::ping`] to verify the decoder is reachable.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dtiw385::Decoders;
    ///
    /// let decoder = Decoders::connect([192, 168, 1, 10], 8080);
    /// ```
    pub fn connect(ip: impl Into<Ipv4Addr>, port: u16) -> Decoder {
        Decoder::new(ip, port)
    }

    /// Prepares a network scan over the given IP and port ranges.
    ///
    /// Call [`DecoderSearch::find`] to execute the scan.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dtiw385::Decoders;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let mut rx = Decoders::search(
    ///     [192, 168, 1, 1]..=[192, 168, 1, 254],
    ///     8080u16..=8080u16,
    /// )
    /// .with_concurrency(50)
    /// .find();
    ///
    /// while let Some(decoder) = rx.recv().await {
    ///     println!("Found: {}", decoder.ip());
    /// }
    /// # }
    /// ```
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

/// A configured network scan, ready to execute.
///
/// Built via [`Decoders::search`].
pub struct DecoderSearch {
    ips: RangeInclusive<Ipv4Addr>,
    ports: RangeInclusive<u16>,
    timeout: u64,
    concurrency: usize,
}

impl DecoderSearch {
    fn new(ips: RangeInclusive<Ipv4Addr>, ports: RangeInclusive<u16>) -> Self {
        Self {
            ips,
            ports,
            timeout: DEFAULT_CLIENT_TIMEOUT_MS,
            concurrency: DEFAULT_CONCURRENCY,
        }
    }

    /// Sets the timeout used when probing each address, in milliseconds.
    ///
    /// Defaults to `250ms`. Lower values speed up the scan but may miss
    /// slow-responding devices.
    pub fn with_timeout(mut self, millis: u64) -> Self {
        self.timeout = millis;
        self
    }

    /// Sets the maximum number of concurrent probes during the scan.
    ///
    /// Defaults to `20`. Raise this value to scan faster at the cost of
    /// higher network and CPU usage.
    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Executes the scan and streams responding decoders through a channel.
    ///
    /// The scan runs in a background Tokio task. The receiver yields each
    /// [`Decoder`] as soon as it responds, without waiting for the full scan
    /// to complete.
    ///
    /// The channel closes automatically once all addresses have been probed.
    pub fn find(&self) -> mpsc::Receiver<Decoder> {
        let (tx, rx) = mpsc::channel(32);
        let sem = Arc::new(Semaphore::new(self.concurrency));

        let start = u32::from(*self.ips.start());
        let end = u32::from(*self.ips.end());
        let ports = self.ports.clone();
        let timeout = self.timeout;
        let total = (end - start + 1) as usize * (ports.clone().count());

        info!(
            start = %Ipv4Addr::from(start),
            end = %Ipv4Addr::from(end),
            ports = ?ports,
            concurrency = self.concurrency,
            total_addresses = total,
            "starting network scan"
        );

        tokio::spawn(async move {
            let mut tasks = JoinSet::new();

            for ip_u32 in start..=end {
                let ip = Ipv4Addr::from(ip_u32);
                for port in ports.clone() {
                    let tx = tx.clone();

                    // Acquire a permit before spawning to cap active tasks at `concurrency`.
                    // The permit is moved into the task and released when the task completes.
                    let permit = sem.clone().acquire_owned().await.unwrap();

                    tasks.spawn(async move {
                        let _permit = permit;

                        let decoder = Decoder::new(ip, port).with_timeout(timeout);
                        debug!(socket = %decoder.socket, "probing");

                        if decoder.ping().await {
                            info!(socket = %decoder.socket, "decoder found");
                            let _ = tx.send(decoder).await;
                        }
                    });
                }
            }

            // Drain completed tasks so JoinSet doesn't grow unboundedly.
            while tasks.join_next().await.is_some() {}

            info!("scan complete");
        });

        rx
    }
}

/// A handle to a single decoder on the network.
///
/// Exposes remote control commands (key press, hold, release) and device
/// information queries. All methods communicate over HTTP and are async.
///
/// Obtained either via [`Decoders::connect`] for a known address, or streamed
/// from [`DecoderSearch::find`] after a network scan.
///
/// # Example
///
/// ```no_run
/// use dtiw385::{Decoders, key::Key};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let decoder = Decoders::connect([192, 168, 1, 10], 8080);
///
///     let info = decoder.infos().await?;
///     println!("Name: {}", info.friendly_name);
///     println!("MAC:  {}", info.mac_address);
///
///     // Simulate pressing and releasing the OK key
///     decoder.hold(Key::Ok).await?;
///     decoder.release(Key::Ok).await?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Decoder {
    socket: SocketAddrV4,
    client: Client,
}

#[cfg(feature = "serializable")]
impl Serialize for Decoder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Decoder", 2)?;
        state.serialize_field("ip", &self.socket.ip().to_string())?;
        state.serialize_field("port", &self.socket.port())?;
        state.end()
    }
}

/// Derefs to [`SocketAddrV4`] so callers can access `.ip()` and `.port()` directly.
impl Deref for Decoder {
    type Target = SocketAddrV4;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

impl Decoder {
    fn new(ip: impl Into<Ipv4Addr>, port: u16) -> Self {
        let socket = SocketAddrV4::new(ip.into(), port);
        let client = build_client(DEFAULT_CLIENT_TIMEOUT_MS);
        Self { socket, client }
    }

    /// Overrides the HTTP request timeout for this decoder.
    ///
    /// Useful when the network conditions differ from the scan defaults.
    /// Defaults to `250ms`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dtiw385::Decoders;
    ///
    /// let decoder = Decoders::connect([192, 168, 1, 10], 8080)
    ///     .with_timeout(1000); // 1 second
    /// ```
    pub fn with_timeout(mut self, millis: u64) -> Self {
        self.client = build_client(millis);
        self
    }

    /// Fetches device information from the decoder.
    ///
    /// Returns metadata such as the friendly name, MAC address, standby state,
    /// and current playback status.
    ///
    /// # Errors
    ///
    /// Returns [`DecoderError`] if the request fails or the decoder reports
    /// a non-zero response code.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dtiw385::Decoders;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let decoder = Decoders::connect([192, 168, 1, 10], 8080);
    /// let info = decoder.infos().await?;
    /// println!("Name: {}", info.friendly_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn infos(&self) -> Result<ApiInfosData, DecoderError> {
        debug!(socket = %self.socket, "fetching infos");
        self.request::<ApiInfosData>(10, 0, 0).await
    }

    /// Sends a key press (down + up) to the decoder.
    ///
    /// Equivalent to a short physical button press. For finer control,
    /// use [`hold`](Self::hold) followed by [`release`](Self::release).
    ///
    /// # Errors
    ///
    /// Returns [`DecoderError`] if the request fails or the decoder reports
    /// a non-zero response code.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dtiw385::{Decoders, key::Key};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let decoder = Decoders::connect([192, 168, 1, 10], 8080);
    /// decoder.press(Key::Ok).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn press(&self, key: impl Into<u16>) -> Result<(), DecoderError> {
        let key = key.into();
        debug!(socket = %self.socket, key, "press");
        self.request::<ApiEmpty>(1, key, MODE_PRESS)
            .await
            .map(|_| ())
    }

    /// Sends a key hold (down only) to the decoder.
    ///
    /// The key remains held until [`release`](Self::release) is called.
    /// Useful for long-press actions or continuous scrolling.
    ///
    /// # Errors
    ///
    /// Returns [`DecoderError`] if the request fails or the decoder reports
    /// a non-zero response code.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dtiw385::{Decoders, key::Key};
    /// use std::time::Duration;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let decoder = Decoders::connect([192, 168, 1, 10], 8080);
    /// decoder.hold(Key::Up).await?;
    /// tokio::time::sleep(Duration::from_secs(1)).await;
    /// decoder.release(Key::Up).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn hold(&self, key: impl Into<u16>) -> Result<(), DecoderError> {
        let key = key.into();
        debug!(socket = %self.socket, key, "hold");
        self.request::<ApiEmpty>(1, key, MODE_HOLD)
            .await
            .map(|_| ())
    }

    /// Sends a key release (up only) to the decoder.
    ///
    /// Should be called after [`hold`](Self::hold) to complete the key event.
    ///
    /// # Errors
    ///
    /// Returns [`DecoderError`] if the request fails or the decoder reports
    /// a non-zero response code.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dtiw385::{Decoders, key::Key};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let decoder = Decoders::connect([192, 168, 1, 10], 8080);
    /// decoder.hold(Key::Ok).await?;
    /// decoder.release(Key::Ok).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn release(&self, key: impl Into<u16>) -> Result<(), DecoderError> {
        let key = key.into();
        debug!(socket = %self.socket, key, "release");
        self.request::<ApiEmpty>(1, key, MODE_RELEASE)
            .await
            .map(|_| ())
    }

    /// Returns `true` if the decoder responds to a status request.
    ///
    /// Used internally during network scans to filter reachable devices.
    pub async fn ping(&self) -> bool {
        let url = self.build_url(10, 0, 0);
        self.client.get(&url).send().await.is_ok()
    }

    /// Sends an HTTP GET request to the decoder and deserializes the JSON response.
    ///
    /// All public commands funnel through this method. Returns an error if the
    /// HTTP request fails, the response is not valid JSON, or the decoder
    /// returns a non-zero `response_code`.
    async fn request<D: DeserializeOwned>(
        &self,
        operation: u8,
        key: u16,
        mode: u8,
    ) -> Result<D, DecoderError> {
        let url = self.build_url(operation, key, mode);
        let result = self
            .client
            .get(url)
            .send()
            .await?
            .json::<ApiResponse<D>>()
            .await?
            .result;

        if result.response_code != "0" {
            warn!(
                socket = %self.socket,
                code = %result.response_code,
                message = %result.message,
                "decoder returned an error"
            );
            return Err(DecoderError::InvalidResponse(result.message));
        }

        Ok(result.data)
    }

    /// Builds the HTTP URL for a given operation, key, and mode.
    fn build_url(&self, operation: u8, key: u16, mode: u8) -> String {
        format!(
            "http://{}/remoteControl/cmd?operation={}&key={}&mode={}",
            self.socket, operation, key, mode
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::net::Ipv4Addr;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    fn decoder_from(server: &MockServer) -> Decoder {
        Decoders::connect(
            server
                .address()
                .ip()
                .to_string()
                .parse::<Ipv4Addr>()
                .unwrap(),
            server.address().port(),
        )
    }

    #[tokio::test]
    async fn infos_success() {
        let server = MockServer::start().await;

        let body = json!({
          "result": {
            "responseCode": "0",
            "message": "ok",
            "data": {
              "osdContext": "",
              "playedMediaType": "NA",
              "playedMediaState": "NA",
              "playedMediaId": "",
              "playedMediaContextId": "",
              "playedMediaPosition": "NA",
              "timeShiftingState": "NA",
              "macAddress": "20:9A:7D:D6:56:B7",
              "wolSupport": "0",
              "friendlyName": "Decodeur TV UHD",
              "activeStandbyState": "0",
              "npvrSupport": "0"
            }
          }
        });

        Mock::given(method("GET"))
            .and(path("/remoteControl/cmd"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let decoder = decoder_from(&server);

        let info = decoder.infos().await.unwrap();

        assert_eq!(info.friendly_name, "Decodeur TV UHD");
        assert_eq!(info.mac_address, "20:9A:7D:D6:56:B7");
    }

    #[tokio::test]
    async fn press_success() {
        let server = MockServer::start().await;

        let body = json!({
            "result": {
                "responseCode": "0",
                "message": "ok",
                "data": {}
            }
        });

        Mock::given(method("GET"))
            .and(path("/remoteControl/cmd"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let decoder = decoder_from(&server);

        let res = decoder.press(key::Key::Ok).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn hold_and_release_success() {
        let server = MockServer::start().await;

        let body = json!({
            "result": {
                "responseCode": "0",
                "message": "ok",
                "data": {}
            }
        });

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let decoder = decoder_from(&server);

        let hold = decoder.hold(key::Key::Up).await;
        let release = decoder.release(key::Key::Up).await;

        assert!(hold.is_ok());
        assert!(release.is_ok());
    }

    #[tokio::test]
    async fn api_error_returns_invalid_response() {
        let server = MockServer::start().await;

        let body = json!({
            "result": {
                "responseCode": "1",
                "message": "error",
                "data": {}
            }
        });

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let decoder = decoder_from(&server);

        let res = decoder.press(key::Key::Ok).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn ping_success() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let decoder = decoder_from(&server);

        assert!(decoder.ping().await);
    }
}
