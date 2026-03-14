use std::net::Ipv4Addr;

use reqwest::Client;

pub struct Decoder {
    pub(crate) ip: Ipv4Addr,
    pub(crate) port: u16,
    pub(crate) client: Client,
}

impl Decoder {
    pub fn new(ip: Ipv4Addr) -> Self {
        Self {
            ip,
            port: 8080,
            client: Client::builder()
                .timeout(std::time::Duration::from_millis(250))
                .build()
                .unwrap(),
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }
}
